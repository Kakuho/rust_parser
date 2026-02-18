#/bin/bash

cargo build
rust-gdb ./target/debug/rust_parser
