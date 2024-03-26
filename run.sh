#!/usr/bin/env bash

# NOTE: this script should only be used for development purposes

# https://stackoverflow.com/a/2173421
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)

cd "$dir/static"
npm install
npm run watch &
cd ..
diesel setup
cargo watch -- cargo run --release --features admin,static
