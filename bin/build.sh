#! /bin/bash 

set -e
cd $(dirname "$0")
bash build-wasm.sh
cd ../packages/game2048
npm run build
