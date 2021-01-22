# logram [![crates.io](https://img.shields.io/crates/v/logram.svg)](https://crates.io/crates/logram)

Utility that takes logs from anywhere and sends them to Telegram. Supports log collection from files, journald and docker containers. More about available log sources see at [`docs/log_sources.md`](docs/log_sources.md).

## Usage
1. Create a Telegram bot via [@BotFather](https://t.me/BotFather)
2. Download the [latest logram release](https://github.com/mxseev/logram/releases/tag/latest)
3. Install it:
  - .deb based Linux: `sudo dpkg -i logram_..._amd64.deb`
  - .rpm based Linux: `sudo rpm -i logram_..._amd64.rpm`
4. Find out the chat id with your bot (see [`docs/chat_id.md`](docs/chat_id.md))
5. Change the config (`/etc/logram.yaml` in Linux) (see [`docs/config.md`](docs/config.md))
6. Run logram: `sudo systemctl start logram`
7. ... and add it to the autostart: `sudo systemctl enable logram`

Also logram can work with log, see [`docs/lib.md`](docs/lib.md) for detals.
