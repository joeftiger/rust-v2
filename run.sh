#!/bin/bash
#SBATCH --job-name="Path"
#SBATCH --time=12:00:00
#SBATCH --mem-per-cpu=200M
#SBATCH --cpus-per-task=64
#SBATCH --partition=epyc2
#SBATCH --mail-user=
#SBATCH --mail-type=end,fail

SCENE=./images/refracting-spheres/20000

export RUST_LOG=info

srun ./spectral $SCENE
