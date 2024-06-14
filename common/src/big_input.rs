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
