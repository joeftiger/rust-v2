#!/bin/bash
#SBATCH --job-name="Build"
#SBATCH --time=00:10:00
#SBATCH --mem-per-cpu=2G
#SBATCH --cpus-per-task=4
#SBATCH --partition=epyc2
#SBATCH --mail-user=
#SBATCH --mail-type=fail

cargo clean

cargo build --bin rust_v2 --release --features watertight-mesh
cp target/release/rust_v2 ./spectral

cargo build --bin rust_v2_error --release --features watertight-mesh
cp target/release/rust_v2_error ./error
