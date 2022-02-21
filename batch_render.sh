#!/bin/bash

export RUST_LOG=info

echo -n "Directory containing scene files: "
read dir

for scene in "$dir"*.ron
do
	./spectral $scene
done
