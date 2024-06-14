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
use sp1_prover::{utils::get_cycles, SP1Prover, SP1Stdin};
use tracing::info;

use crate::{metric, VmArgs};

const ELF: &[u8] = include_bytes!("../../guest-sp1/elf/riscv32im-succinct-zkvm-elf");

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
    info!("-=-=-=-=-=- Measuring SP1 -=-=-=-=-=-");

    let input = BenchmarkInput::from(input);

    info!("Setting env vars ...");
    {
        if let Some(sp1_save_disk_threshold) = vm_args.sp1_save_disk_threshold {
            std::env::set_var(
                "SAVE_DISK_THRESHOLD",
                format!("{}", sp1_save_disk_threshold),
            );
        }

        if let Some(sp1_shard_batch_size) = vm_args.sp1_shard_batch_size {
            std::env::set_var("SHARD_BATCH_SIZE", format!("{}", sp1_shard_batch_size));
        }

        // shard_size
        std::env::set_var("SHARD_SIZE", format!("{}", 1 << vm_args.segment_size));
    }

    info!("Setting up stdin ...");
    let stdin = {
        let mut stdin = SP1Stdin::new();
        stdin.write(&input);
        for val in &extra_input {
            stdin.write(val);
        }

        stdin
    };

    info!("Creating prover ...");
    let (prover, pk, vk) = metric::measure(&mut metrics.prover_create_millis, || {
        let prover = SP1Prover::new();
        let (pk, vk) = prover.setup(ELF);
        Ok((prover, pk, vk))
    })?;

    info!("Running executor ...");
    {
        let mut session = metric::measure(&mut metrics.exec_millis, || {
            Ok(SP1Prover::execute(ELF, &stdin))
        })?;

        info!("  Reading session metadata ...");
        metrics.exec_user_cycles = Some(get_cycles(ELF, &stdin));

        info!("  Reading guest output ...");
        let output = session.read::<X::Output>();
        metrics.exec_output = Some(format!("{:?}", output));
    }

    info!("Segmented proof workflow ...");
    let proof = {
        info!("  Proving ...");
        let mut proof = metric::measure(&mut metrics.segmented_prove_millis, || {
            Ok(prover.prove_core(&pk, &stdin))
        })?;

        info!("  Reading proof metadata ...");
        metrics.segmented_poof_segments = Some(proof.proof.0.len() as u64);
        metrics.segmented_proof_kind = Some("SP1CoreProofData".into());

        info!("  Reading guest output ...");
        {
            let output = proof.public_values.read::<X::Output>();
            metrics.segmented_proof_output = Some(format!("{:?}", output));
        }

        info!("  Verifying proof ...");
        metric::measure(&mut metrics.segmented_verify_millis, || {
            prover.verify(&proof.proof, &vk).context("Segmented verify")
        })?;

        info!("  Measuring proof size ...");
        {
            // Measure proof w metadata size
            let proof_w_metadata_bytes =
                bincode::serialize(&proof).context("Seralize segmented proof w metadata")?;
            metrics.segmented_proof_w_metadata_bytes = Some(proof_w_metadata_bytes.len() as u128);

            // Measure proof size
            let proof_bytes =
                bincode::serialize(&proof.proof).context("Serialize segmented inner proof")?;
            metrics.segmented_proof_bytes = Some(proof_bytes.len() as u128);
        }

        proof
    };

    info!("Reduced proof workflow ...");
    {
        info!("  Proving ...");
        let proof = metric::measure(&mut metrics.reduced_prove_millis, || {
            Ok(prover.compress(&vk, proof, vec![]))
        })?;

        info!("  Reading proof metadata ...");
        metrics.reduced_proof_kind = Some("SP1ReduceProof<BabyBearPoseidon2>".into());

        // Not supported!
        //
        // info!("  Verifying proof ...");
        // metric::measure(&mut metrics.reduced_verify_millis, || {
        //     prover
        //         .verify_compressed(&proof, &vk)
        //         .context("Reduced verify")
        // })?;

        info!("  Measuring proof size ...");
        {
            // Measure proof w metadata size
            let proof_w_metadata_bytes =
                bincode::serialize(&proof).context("Seralize reduced proof w metadata")?;
            metrics.reduced_proof_w_metadata_bytes = Some(proof_w_metadata_bytes.len() as u128);

            // Measure proof size
            let proof_bytes =
                bincode::serialize(&proof.proof).context("Serialize reduced inner proof")?;
            metrics.reduced_proof_bytes = Some(proof_bytes.len() as u128);
        }
    }

    info!("Done");

    Ok(())
}
