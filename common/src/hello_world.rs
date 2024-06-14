use clap::Args;
use serde::{Deserialize, Serialize};

use crate::{BenchmarkInput, Runnable};

#[derive(Args, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Param {}

impl Param {
    pub fn input(&self) -> Input {
        Input {}
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Input {}

impl From<Input> for BenchmarkInput {
    fn from(x: Input) -> Self {
        Self::HelloWorld(x)
    }
}

impl Runnable for Input {
    type Output = ();

    #[cfg(feature = "guest")]
    fn run(self) -> anyhow::Result<Self::Output> {
        Ok(())
    }
}
