use clap::Args;
use serde::{Deserialize, Serialize};

use crate::{BenchmarkInput, Runnable};

#[derive(Args, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Param {
    pub words: u32,
}

impl Param {
    pub fn input(&self) -> (Input, Vec<u32>) {
        let input = Input { words: self.words };

        let extra_input: Vec<u32> = (0..self.words).collect();

        (input, extra_input)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Input {
    pub words: u32,
}

impl From<Input> for BenchmarkInput {
    fn from(x: Input) -> Self {
        Self::BigInputPush(x)
    }
}

impl Runnable for Input {
    type Output = u32;

    #[cfg(feature = "guest")]
    fn run(self) -> anyhow::Result<Self::Output> {
        let mut words: Vec<u32> = Vec::with_capacity(self.words as usize);

        for _ in 0..self.words {
            #[cfg(feature = "r0")]
            {
                words.push(risc0_zkvm::guest::env::read());
            }

            #[cfg(feature = "sp1")]
            {
                words.push(sp1_zkvm::io::read());
            }
        }

        Ok(words.into_iter().sum())
    }
}
