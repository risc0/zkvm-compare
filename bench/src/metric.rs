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

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ExperimentId {
    pub experiment_id: String,
}

impl std::fmt::Display for ExperimentId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        f.write_str(&self.experiment_id)
    }
}

impl ExperimentId {
    pub fn new() -> Self {
        ExperimentId {
            experiment_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meta {
    pub machine_tag: String,
    pub benchmark_name: String,
    pub benchmark_size: u32,
    pub vm: String,
    pub segment_size: u32,
    pub experiment_id: String,
    pub datetime: String,
    pub r0_feature_flags: Option<String>,
    pub r0_version: String,
    pub sp1_save_disk_threshold: Option<u32>,
    pub sp1_shard_batch_size: Option<u32>,
    pub cli_args: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Metrics {
    pub error_string: Option<String>,
    pub input_bytes: Option<u128>,

    pub exec_millis: Option<u128>,
    pub exec_user_cycles: Option<u64>,
    pub exec_output: Option<String>,

    pub prover_create_millis: Option<u128>,

    pub segmented_prove_millis: Option<u128>,
    pub segmented_poof_segments: Option<u64>,
    pub segmented_proof_kind: Option<String>,
    pub segmented_proof_output: Option<String>,
    pub segmented_verify_millis: Option<u128>,
    pub segmented_proof_w_metadata_bytes: Option<u128>,
    pub segmented_proof_bytes: Option<u128>,

    pub reduced_prove_millis: Option<u128>,
    pub reduced_proof_kind: Option<String>,
    pub reduced_verify_millis: Option<u128>,
    pub reduced_proof_w_metadata_bytes: Option<u128>,
    pub reduced_proof_bytes: Option<u128>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Measurement(pub Meta, pub Metrics);

pub fn measure<X, F>(metric: &mut Option<u128>, f: F) -> Result<X>
where
    F: FnOnce() -> Result<X>,
{
    let start = std::time::Instant::now();
    let out = f()?;
    let elapsed = start.elapsed().as_millis();
    *metric = Some(elapsed);
    Ok(out)
}
