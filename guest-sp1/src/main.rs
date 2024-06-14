#![no_main]
sp1_zkvm::entrypoint!(main);

use common::BenchmarkInput;

pub fn main() {
    let bench = sp1_zkvm::io::read::<BenchmarkInput>();
    bench.run().unwrap();
}
