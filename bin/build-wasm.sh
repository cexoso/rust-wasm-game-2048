#! /bin/bash 

set -e
cd $(dirname "$0")
cd ../packages/core
wasm-pack build --release --out-dir ../core-wasm
ls -lh ../core-wasm
