#!/bin/bash

PORT=${1:-8000}

./build-wasm.sh

python3 -m http.server $PORT
