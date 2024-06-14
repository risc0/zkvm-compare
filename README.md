# zkvm-compare

Requires [RISC Zero](https://github.com/risc0/risc0?tab=readme-ov-file#getting-started) and [SP1](https://succinctlabs.github.io/sp1/getting-started/install.html).

## Build the sp1 guest

```console
(cd guest-sp1 ; cargo prove build)
```

## Collect measurements

The general command structure is: `cargo run --release -F <FEATURE_FLAG> -- measure --segment-size=<SEGMENT_SIZE> <MACHINE_TAG> <VM> <BENCH>`.

Example:

```console
cargo run --release -F metal -- measure --segment-size=20 tim-mbp-m3 r0 hello-world
```

During execution, data will be written to log and CSV files in the `data` directory.


## Run all experiments

The general command structure is: `bash -x run-all.sh <FEATURE_FLAG> <MACHINE_TAG> <VM> <SEGMENT_SIZE> >> run.log 2>&1`.

Examples:

```console
bash -x run-all.sh metal mbp-m3 r0 20 >> run.log 2>&1
bash -x run-all.sh metal mbp-m3 sp1 20 >> run.log 2>&1

bash -x run-all.sh cuda g4dn.xlarge r0 20 >> run.log 2>&1

bash -x run-all.sh cuda g6.xlarge r0 21 >> run.log 2>&1

bash -x run-all.sh cuda g6.16xlarge r0 21 >> run.log 2>&1
bash -x run-all.sh cuda g6.16xlarge sp1 21 >> run.log 2>&1

bash -x run-all.sh default r7i.16xlarge sp1 22 >> run.log 2>&1
bash -x run-all.sh default r7i.16xlarge sp1 21 >> run.log 2>&1
```
