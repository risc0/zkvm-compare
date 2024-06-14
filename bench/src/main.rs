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
use clap::Parser;

mod cli;
mod metric;
mod r0;
mod sp1;

use cli::Cli;

pub struct VmArgs {
    pub segment_size: u32,
    pub sp1_save_disk_threshold: Option<u32>,
    pub sp1_shard_batch_size: Option<u32>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.run()?;

    Ok(())
}
