#!/bin/env sh

cargo build --release --jobs 1
cargo run --release
