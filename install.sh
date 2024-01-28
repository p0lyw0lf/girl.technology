#!/bin/env sh

dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)
sudo ln -s "$dir/bot-girl-technology.service" /etc/systemd/system/bot-girl-technology.service
sudo useradd -m -U bot-girl-technology -G ubuntu

sudo apt install postgresql postgresql-contrib libpq-dev

cd "$dir"
cargo install diesel_cli --no-default-features --features postgres
diesel setup
