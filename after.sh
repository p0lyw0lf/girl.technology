#!/bin/env sh

systemctl --user stop girl.technology.service
# It seems that github takes a second to update the latest release URL, even
# after the release webhook is fired. So, we need to wait a few seconds before
# attempting to fetch
sleep 5
# cargo-dist makes this really easy, thanks axo.dev!
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/p0lyw0lf/girl.technology/releases/latest/download/girl_technology-installer.sh | sh
systemctl --user daemon-reload
systemctl --user start girl.technology.service
