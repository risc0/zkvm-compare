// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::{Context, Result};
use common::*;
use guest_r0::{GUEST_R0_ELF, GUEST_R0_ID};
use risc0_zkvm::{default_executor, default_prover, ExecutorEnv, ProverOpts};
use tracing::info;

use crate::{metric, VmArgs};

fn proof_kind(receipt: &risc0_zkvm::Receipt) -> String {
    match receipt.inner {
        risc0_zkvm::InnerReceipt::Composite(_) => "composite".into(),
        risc0_zkvm::InnerReceipt::Succinct(_) => "succinct".into(),
        risc0_zkvm::InnerReceipt::Compact(_) => "compact".into(),
        risc0_zkvm::InnerReceipt::Fake { .. } => "fake".into(),
        _ => "unknown".into(),
    }
}

pub fn measure<X>(
    metrics: &mut metric::Metrics,
    vm_args: VmArgs,
    input: X,
    extra_input: Vec<u32>,
) -> Result<()>
where
    X: Clone + Runnable + serde::Serialize,
    BenchmarkInput: From<X>,
{
    info!("-=-=-=-=-=- Measuring R0 -=-=-=-=-=-");

    let input = BenchmarkInput::from(input);

    info!("Creating prover ...");
    let prover = metric::measure(&mut metrics.prover_create_millis, || Ok(default_prover()))?;

    info!("Running executor ...");
    {
        let mut env = ExecutorEnv::builder();
        env.write(&input).context("Write input for executor")?;
        for val in &extra_input {
            env.write(val).context("Write input for executor")?;
        }
        env.segment_limit_po2(vm_args.segment_size);
        let env = env.build().context("Build env for executor")?;

        let session = metric::measure(&mut metrics.exec_millis, || {
            default_executor()
                .execute(env, GUEST_R0_ELF)
                .context("Run executor")
        })?;

        info!("  Reading session metadata ...");
        metrics.exec_user_cycles = Some(
            session
                .segments
                .iter()
                .map(|segment| segment.cycles as u64)
                .sum(),
        );

        info!("  Reading guest output ...");
        let output = session
            .journal
            .decode::<X::Output>()
            .context("Decode executor journal")?;
        metrics.exec_output = Some(format!("{:?}", output));
    }

    info!("Segmented proof workflow ...");
    let proof = {
        info!("  Proving ...");
        let proof_info = {
            let mut env = ExecutorEnv::builder();
            env.write(&input)
                .context("Write input for segmented proof")?;
            for val in &extra_input {
                env.write(val).context("Write input for segmented proof")?;
            }
            env.segment_limit_po2(vm_args.segment_size);
            let env = env.build().context("Build env for segmented proof")?;

            let proof_info = metric::measure(&mut metrics.segmented_prove_millis, || {
                prover.prove(env, GUEST_R0_ELF).context("Segmented prove")
            })?;

            proof_info
        };

        let receipt = proof_info.receipt;

        info!("  Reading proof metadata ...");
        metrics.segmented_poof_segments = Some(proof_info.stats.segments as u64);
        metrics.segmented_proof_kind = Some(proof_kind(&receipt));

        info!("  Reading guest output ...");
        {
            let output = receipt
                .journal
                .decode::<X::Output>()
                .context("Decode segmented journal")?;
            metrics.segmented_proof_output = Some(format!("{:?}", output));
        }

        info!("  Verifying proof ...");
        metric::measure(&mut metrics.segmented_verify_millis, || {
            receipt
                .verify(GUEST_R0_ID)
                .context("Verify segmented proof")
        })?;

        info!("  Measuring proof size ...");
        {
            // Measure receipt size
            let receipt_bytes =
                bincode::serialize(&receipt).context("Serialize segmented receipt")?;
            metrics.segmented_proof_w_metadata_bytes = Some(receipt_bytes.len() as u128);

            // Measure inner receipt size
            let inner_receipt_bytes =
                bincode::serialize(&receipt.inner).context("Serialize segmented inner receipt")?;
            metrics.segmented_proof_bytes = Some(inner_receipt_bytes.len() as u128);
        }

        receipt
    };

    info!("Reduced proof workflow ...");
    let _proof = {
        info!("  Proving ...");
        let receipt = metric::measure(&mut metrics.reduced_prove_millis, || {
            prover
                .compress(&ProverOpts::succinct(), &proof)
                .context("Reduced prove")
        })?;

        info!("  Reading proof metadata ...");
        metrics.reduced_proof_kind = Some(proof_kind(&receipt));

        info!("  Verifying proof ...");
        metric::measure(&mut metrics.reduced_verify_millis, || {
            receipt.verify(GUEST_R0_ID).context("Verify reduced proof")
        })?;

        info!("  Measuring proof size ...");
        {
            // Measure receipt size
            let receipt_bytes = bincode::serialize(&receipt).context("Serialize receipt")?;
            metrics.reduced_proof_w_metadata_bytes = Some(receipt_bytes.len() as u128);

            // Measure inner receipt size
            let inner_receipt_bytes =
                bincode::serialize(&receipt.inner).context("Serialize inner receipt")?;
            metrics.reduced_proof_bytes = Some(inner_receipt_bytes.len() as u128);
        }

        receipt
    };

    info!("Done");

    Ok(())
}
