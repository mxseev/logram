# logram - pipe log updates to Telegram [![crates.io](https://img.shields.io/crates/v/logram.svg)](https://crates.io/crates/logram)

## Use
1. Install Rust via [Rustup](https://rustup.rs)
2. Install logram: `cargo install logram`
3. Create bot via [@BotFather](https://t.me/BotFather)
4. Send any message to bot
5. Run logram in `echo id` mode: `logram echo_id --token=...`
6. Use received chat id in config
7. Write config from example
8. Run logram `logram --config=...`
9. Create systemd service if needed

## Config example
```yaml
telegram:
  chat_id: 12345678 # chat id
  token: 123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11 # bot token
  # if several records appear with the same title at the same time
  # logram will not send multiple messages, but simply change the last message.
  # debounce_timeout allows you to set a timeout after which logram will send a new message
  # (even if the headers are the same)
  # default 10 seconds, use 0 to disable it
  debounce_timeout: 10

sources:
  fs: 
    entries: # paths to watching files or dirs
      - /tmp/log_file
      
  journald:
    units: # names of systemd units for watching
      - docker.service
      - nginx.service
```

## Systemd service
1. Create link: `sudo ln -s /home/<user>/.cargo/bin/logram /usr/bin/logram`
2. Copy config to `/etc/logram.yaml`
3. Copy `logram.service` to `/etc/systemd/system`
4. Reload services: `sudo systemctl daemon-reload`
5. Enable service: `sudo systemctl enable logram`
6. Run service: `sudo systemctl start logram`

## Usage with `log`
1. Load `logram` as library
```toml
[dependencies]
logram = "1.1"
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
