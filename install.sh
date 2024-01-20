#!/bin/env sh

dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)
sudo ln -s "$dir/bot-girl-technology.service" /etc/systemd/system/bot-girl-technology.service
