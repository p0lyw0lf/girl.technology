#!/usr/bin/env bash

dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)
cd "$dir/static"
npm install
npm run build
cd ..
# Depends on `./install.sh` having run
/home/ubuntu/.cargo/bin/diesel setup
# Depends on `./after.sh` having run
/home/ubuntu/.cargo/bin/girl_technology 127.0.0.1 3001
