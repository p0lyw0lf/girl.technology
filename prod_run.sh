#!/bin/env sh

dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)
cd "$dir/static"
npm install
npm run build
cd ..
diesel setup
# Depends on `./after.sh` having run
/home/bot-github_webhook_watcher/.cargo/bin/girl_technology
