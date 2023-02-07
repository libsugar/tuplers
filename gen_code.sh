#!/bin/bash

cd ./code_gen/
cargo build --release
cd ..
./target/release/code_gen

for rs in ./tuples/src/gen/*.rs; do   rustfmt "${rs}"; done
