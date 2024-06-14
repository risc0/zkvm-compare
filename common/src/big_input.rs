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
    pub words: u32,
}

impl Param {
    pub fn input(&self) -> Input {
        Input {
            words: (0..self.words).collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Input {
    pub words: Vec<u32>,
}

impl From<Input> for BenchmarkInput {
    fn from(x: Input) -> Self {
        Self::BigInput(x)
    }
}

impl Runnable for Input {
    type Output = u32;

    #[cfg(feature = "guest")]
    fn run(self) -> anyhow::Result<Self::Output> {
        Ok(self.words.into_iter().sum())
    }
}
