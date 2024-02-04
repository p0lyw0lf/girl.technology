#!/bin/env sh

sudo -n systemctl stop bot-girl-technology.service
# Thanks, cargo-dist!
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/p0lyw0lf/girl.technology/releases/latest/download/girl_technology-installer.sh | sh
sudo -n systemctl daemon-reload
sudo -n systemctl start bot-girl-technology.service
