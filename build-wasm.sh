#!/bin/bash

if ! command -v wasm-pack &> /dev/null
then
    cargo install wasm-pack
fi

wasm-pack build --target web
