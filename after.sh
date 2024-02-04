#!/bin/env sh

# Thanks, cargo-dist!
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/p0lyw0lf/girl.technology/releases/latest/download/girl_technology-installer.sh | sh
sudo -n systemctl daemon-reload
sudo -n systemctl restart bot-girl-technology.service
