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
        Self::Sort(x)
    }
}

impl Runnable for Input {
    type Output = u32;

    #[cfg(feature = "guest")]
    fn run(self) -> anyhow::Result<Self::Output> {
        use rand::prelude::*;
        use rand_chacha::ChaCha20Rng;

        let mut words: Vec<u32> = Vec::with_capacity(self.n as usize);
        let mut rng = ChaCha20Rng::seed_from_u64(self.n as u64);

        for _ in 0..self.n {
            words.push(rng.gen());
        }

        words.sort();

        let mut out = 0;
        let mut sgn = true;
        for val in words {
            if sgn {
                out = out + val;
            } else {
                out = out - val;
            }

            sgn = !sgn;
        }

        Ok(out)
    }
}
