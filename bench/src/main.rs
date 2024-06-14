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
