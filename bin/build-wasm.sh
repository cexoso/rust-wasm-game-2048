#! /bin/bash 

set -e
cd $(dirname "$0")
cd ../packages/core
wasm-pack build --out-dir ../core-wasm
