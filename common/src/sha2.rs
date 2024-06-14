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

use clap::Args;
use serde::{Deserialize, Serialize};

use crate::{BenchmarkInput, Runnable};

#[derive(Args, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Param {
    pub n: u32,
}

impl Param {
    pub fn input(&self) -> Input {
        Input { n: self.n }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Input {
    pub n: u32,
}

impl From<Input> for BenchmarkInput {
    fn from(x: Input) -> Self {
        Self::Sha2(x)
    }
}

impl Runnable for Input {
    type Output = [u8; 32];

    #[cfg(feature = "guest")]
    fn run(self) -> anyhow::Result<Self::Output> {
        #[cfg(feature = "r0")]
        use sha2_r0::{Digest, Sha256};

        #[cfg(feature = "sp1")]
        use sha2_sp1::{Digest, Sha256};

        let mut out = [0u8; 32];

        for _ in 0..self.n {
            let mut hasher = Sha256::new();
            hasher.update(out);
            let digest = &hasher.finalize();
            out = Into::<[u8; 32]>::into(*digest);
        }

        Ok(out)
    }
}
