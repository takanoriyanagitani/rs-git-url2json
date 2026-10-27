#!/bin/bash

wsm="./git-url2json.wasm"

run_wasmtime(){
  cat /dev/stdin |
    wasmtime \
      run \
      "${wsm}"
}

run_wasi(){
  run_wasmtime
}

input(){
  cargo \
    metadata \
    --format-version 1 |
  jq -r '.packages[].repository' |
  grep -v '^null$' |
  sort -u
}

input |
  run_wasi |
  jq -c
