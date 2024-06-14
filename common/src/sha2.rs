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
