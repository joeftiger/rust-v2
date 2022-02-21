#!/bin/bash
#SBATCH --job-name="Error"
#SBATCH --time=12:00:00
#SBATCH --mem-per-cpu=200M
#SBATCH --cpus-per-task=64
#SBATCH --partition=epyc2
#SBATCH --mail-user=
#SBATCH --mail-type=end,fail

IMAGE=./images/cornell/20000.png

export RUST_LOG=info

srun ./error 1 $IMAGE hero random
