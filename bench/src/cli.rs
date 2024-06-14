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

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use common::*;
use tracing::{error, info};
use tracing_subscriber::{filter, fmt, layer::Layer, prelude::*, Registry};

use crate::{metric::*, VmArgs};

#[derive(Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum Vm {
    R0,
    SP1,
}

impl std::fmt::Display for Vm {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match *self {
            Vm::R0 => f.write_str("r0"),
            Vm::SP1 => f.write_str("sp1"),
        }
    }
}

impl Vm {
    pub fn measure<X>(
        &self,
        metrics: &mut Metrics,
        vm_args: VmArgs,
        input: X,
        extra_input: Vec<u32>,
    ) -> Result<()>
    where
        X: Clone + Runnable + serde::Serialize,
        BenchmarkInput: From<X>,
    {
        // Measure input size
        {
            let input = BenchmarkInput::from(input.clone());
            let base_input_bytes = bincode::serialize(&input)?.len() as u128;
            let extra_input_bytes = extra_input.len() as u128 * 4;
            metrics.input_bytes = Some(base_input_bytes + extra_input_bytes);
        }

        // Measure VM performance
        match self {
            Vm::R0 => crate::r0::measure(metrics, vm_args, input, extra_input),
            Vm::SP1 => crate::sp1::measure(metrics, vm_args, input, extra_input),
        }
    }
}

#[derive(Subcommand)]
pub enum Command {
    Measure {
        machine_tag: String,

        vm: Vm,

        #[arg(long, require_equals = true)]
        segment_size: u32,

        #[arg(long, require_equals = true)]
        sp1_save_disk_threshold: Option<u32>,

        #[arg(long, require_equals = true)]
        sp1_shard_batch_size: Option<u32>,

        #[command(subcommand)]
        benchmark: BenchmarkParam,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(long, require_equals = true, default_value_t = String::from("data"))]
    pub datadir: String,

    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Command::Measure {
                machine_tag,
                vm,
                segment_size,
                sp1_save_disk_threshold,
                sp1_shard_batch_size,
                benchmark,
            } => {
                let vm_args = VmArgs {
                    segment_size: *segment_size,
                    sp1_save_disk_threshold: *sp1_save_disk_threshold,
                    sp1_shard_batch_size: *sp1_shard_batch_size,
                };
                self.measure(machine_tag.clone(), vm, vm_args, benchmark)?
            }
        }
        Ok(())
    }

    pub fn log_file_path(
        &self,
        machine_tag: &String,
        experiment_id: &ExperimentId,
    ) -> std::path::PathBuf {
        [
            &self.datadir,
            machine_tag,
            &format!("{}.log", experiment_id),
        ]
        .iter()
        .collect()
    }

    pub fn measurements_file_path(&self) -> std::path::PathBuf {
        [&self.datadir, "measurements.csv"].iter().collect()
    }

    fn read_measurements(&self) -> Result<Vec<Measurement>> {
        let mut measurements: Vec<Measurement> = Vec::new();

        let csv_file = match std::fs::OpenOptions::new()
            .read(true)
            .open(self.measurements_file_path())
            .context("Open CSV file")
        {
            Ok(csv_file) => csv_file,
            Err(_) => return Ok(measurements),
        };

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv_file);
        for row in rdr.deserialize::<Measurement>() {
            if let Ok(measurement) = row {
                measurements.push(measurement)
            }
        }

        Ok(measurements)
    }

    pub fn write_measurements(&self, measurements: &Vec<Measurement>) -> Result<()> {
        let csv_file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(self.measurements_file_path())
            .context("Create CSV file")?;

        let mut wtr = csv::WriterBuilder::new().from_writer(csv_file);
        for measurement in measurements {
            wtr.serialize(measurement)
                .context("Serialize metric into CSV")?;
        }
        wtr.flush().context("Flush CSV")?;
        Ok(())
    }

    fn setup_experiment(&self, machine_tag: &String) -> Result<ExperimentId> {
        let experiment_id = ExperimentId::new();
        println!("{}", experiment_id);

        let log_file_path = self.log_file_path(machine_tag, &experiment_id);

        std::fs::create_dir_all(log_file_path.parent().unwrap())
            .context("Create data directory")?;

        let log_file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_file_path)
            .context("Create log file")?;

        let subscriber = Registry::default().with(
            fmt::layer()
                .with_ansi(false)
                .with_writer(log_file)
                .with_filter(filter::LevelFilter::from_level(tracing::Level::INFO)),
        );
        tracing::subscriber::set_global_default(subscriber)?;

        Ok(experiment_id)
    }

    fn measure(
        &self,
        machine_tag: String,
        vm: &Vm,
        vm_args: VmArgs,
        benchmark: &BenchmarkParam,
    ) -> Result<()> {
        let experiment_id = self.setup_experiment(&machine_tag)?;
        let mut measurements = self.read_measurements()?;

        let meta = {
            let datetime = chrono::Utc::now().to_rfc3339();

            #[allow(unused_assignments, unused_mut)]
            let mut r0_feature_flags: Option<String> = None;

            #[cfg(feature = "cuda")]
            {
                r0_feature_flags = Some("cuda".into());
            }

            #[cfg(feature = "metal")]
            {
                r0_feature_flags = Some("metal".into());
            }

            Meta {
                machine_tag,
                benchmark_name: benchmark.name(),
                benchmark_size: benchmark.size(),
                vm: vm.to_string(),
                segment_size: vm_args.segment_size,
                experiment_id: experiment_id.to_string(),
                datetime,
                r0_feature_flags,
                r0_version: risc0_zkvm::VERSION.into(),
                sp1_save_disk_threshold: vm_args.sp1_save_disk_threshold,
                sp1_shard_batch_size: vm_args.sp1_shard_batch_size,
                cli_args: std::env::args().collect::<Vec<_>>().join(" "),
            }
        };

        info!("Measurement metadata: {:?}", meta);

        let mut metrics = Metrics::default();

        // Perform the measurement
        let measure_res = match benchmark {
            BenchmarkParam::BigInput(param) => {
                vm.measure(&mut metrics, vm_args, param.input(), vec![])
            }
            BenchmarkParam::BigInputPush(param) => {
                let (input, extra_input) = param.input();
                vm.measure(&mut metrics, vm_args, input, extra_input)
            }
            BenchmarkParam::BigInputVecless(param) => {
                let (input, extra_input) = param.input();
                vm.measure(&mut metrics, vm_args, input, extra_input)
            }
            BenchmarkParam::Fib(param) => vm.measure(&mut metrics, vm_args, param.input(), vec![]),
            BenchmarkParam::HelloWorld(param) => {
                vm.measure(&mut metrics, vm_args, param.input(), vec![])
            }
            BenchmarkParam::Sha2(param) => vm.measure(&mut metrics, vm_args, param.input(), vec![]),
            BenchmarkParam::Sort(param) => vm.measure(&mut metrics, vm_args, param.input(), vec![]),
        };

        if let Err(err) = measure_res {
            error!("Error when taking measurements: {:?}", err);
            metrics.error_string = Some(serde_json::to_string(&format!("{:?}", err))?);
        }

        measurements.push(Measurement(meta, metrics));
        self.write_measurements(&measurements)?;

        Ok(())
    }
}
