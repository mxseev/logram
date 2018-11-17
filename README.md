# logram - push logs updates to Telegram [![crates.io](https://img.shields.io/crates/v/logram.svg)](https://crates.io/crates/logram) [![travis-ci.org](https://api.travis-ci.org/Ralvke/logram.svg?branch=master)](https://travis-ci.org/Ralvke/logram)

## Use
1. Install Rust via [Rustup](https://rustup.rs)
2. Install logram: `cargo install logram`
3. Create bot via [@BotFather](https://t.me/BotFather)
4. Write config from example
4. WIP

## Config example
```yaml

```

## Systemd service
1. Create link: `sudo ln -s /home/<user>/.cargo/bin/logram /usr/bin/logram`
2. Copy config to `/etc/logram.yaml`
3. Copy `logram.service` to `/etc/systemd/system`
4. Reload services: `sudo systemctl daemon-reload`
5. Enable service: `sudo systemctl enable logram`
6. Run service: `sudo systemctl start logram`