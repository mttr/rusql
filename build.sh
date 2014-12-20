#!/bin/bash

cd rust-peg
cargo build
cd ..
RUST_BACTRACE=1 ./rust-peg/target/peg src/parser/sql.rustpeg > src/parser/parser.rs
cargo build
