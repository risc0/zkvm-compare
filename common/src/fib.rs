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
        Self::Fib(x)
    }
}

impl Runnable for Input {
    type Output = u32;

    #[cfg(feature = "guest")]
    fn run(self) -> anyhow::Result<Self::Output> {
        let mut a = 0;
        let mut b = 1;

        for _ in 0..self.n {
            let c = a + b;
            a = b;
            b = c;
        }

        Ok(b)
    }
}
