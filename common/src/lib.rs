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
use clap::Subcommand;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod big_input;
pub mod big_input_push;
pub mod big_input_vecless;
pub mod fib;
pub mod hello_world;
pub mod sha2;
pub mod sort;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Subcommand)]
pub enum BenchmarkParam {
    BigInput(big_input::Param),
    BigInputPush(big_input_push::Param),
    BigInputVecless(big_input_vecless::Param),
    Fib(fib::Param),
    HelloWorld(hello_world::Param),
    Sha2(sha2::Param),
    Sort(sort::Param),
}

impl BenchmarkParam {
    pub fn name(&self) -> String {
        match self {
            BenchmarkParam::BigInput(_) => "big_input".into(),
            BenchmarkParam::BigInputPush(_) => "big_input_push".into(),
            BenchmarkParam::BigInputVecless(_) => "big_input_vecless".into(),
            BenchmarkParam::Fib(_) => "fib".into(),
            BenchmarkParam::HelloWorld(_) => "hello_world".into(),
            BenchmarkParam::Sha2(_) => "sha2".into(),
            BenchmarkParam::Sort(_) => "sort".into(),
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            BenchmarkParam::BigInput(param) => param.words,
            BenchmarkParam::BigInputPush(param) => param.words,
            BenchmarkParam::BigInputVecless(param) => param.words,
            BenchmarkParam::Fib(param) => param.n,
            BenchmarkParam::HelloWorld(_) => 0,
            BenchmarkParam::Sha2(param) => param.n,
            BenchmarkParam::Sort(param) => param.n,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BenchmarkInput {
    BigInput(big_input::Input),
    BigInputPush(big_input_push::Input),
    BigInputVecless(big_input_vecless::Input),
    Fib(fib::Input),
    HelloWorld(hello_world::Input),
    Sha2(sha2::Input),
    Sort(sort::Input),
}

impl BenchmarkInput {
    #[cfg(feature = "guest")]
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            BenchmarkInput::BigInput(x) => commit(&x.run().unwrap()),
            BenchmarkInput::BigInputPush(x) => commit(&x.run().unwrap()),
            BenchmarkInput::BigInputVecless(x) => commit(&x.run().unwrap()),
            BenchmarkInput::Fib(x) => commit(&x.run().unwrap()),
            BenchmarkInput::HelloWorld(x) => commit(&x.run().unwrap()),
            BenchmarkInput::Sha2(x) => commit(&x.run().unwrap()),
            BenchmarkInput::Sort(x) => commit(&x.run().unwrap()),
        }

        Ok(())
    }
}

pub trait Runnable
where
    Self: Sized,
{
    type Output: std::fmt::Debug + DeserializeOwned + Eq + PartialEq + Serialize;

    fn run(self) -> Result<Self::Output> {
        unimplemented!()
    }
}

#[cfg(feature = "guest")]
pub fn commit<X: Serialize>(val: &X) {
    #[cfg(not(any(feature = "r0", feature = "sp1")))]
    compile_error!("feature \"r0\" or feature \"sp1\" must be enabled");

    #[cfg(feature = "r0")]
    {
        risc0_zkvm::guest::env::commit(val);
    }

    #[cfg(feature = "sp1")]
    {
        sp1_zkvm::io::commit(val);
    }
}
