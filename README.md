# logram - pipe log updates to Telegram [![crates.io](https://img.shields.io/crates/v/logram.svg)](https://crates.io/crates/logram)
Logram takes logs from files and systemd services and send them to Telegram.

## Usage
1. Download the [latest release](https://github.com/Ralvke/logram/releases) and install it: `sudo dpkg -i logram_..._amd64.deb`
2. Create bot via [@BotFather](https://t.me/BotFather)
3. Run logram in `echo_id` mode: `logram echo_id --token=...`
4. Send any message to bot and he will answer chat id
5. Change config at `/etc/logram.yaml`
6. Run via systemd: `sudo systemctl start logram`
7. Optionally: enable systemd service (for autostart): `sudo systemctl enable logram`

## Usage with `log`
1. Load `logram` as library
```toml
[dependencies]
logram = "1.2"
```
2. Init logram
```rust
use log::{debug, error, info, log, trace, warn, Level};
use logram;

fn main() {
    logram::init(
        "bot token".to_string(),
        "chat id".to_string(),
        Level::Error,
    )
    .unwrap();

    error!("error");
}
```
Limitations: log records with target starts with `tokio_reactor, hyper, mio, want or reqwest` will be skipped, because [limitations in log](https://github.com/rust-lang-nursery/log/issues/312).
