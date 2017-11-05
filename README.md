# logram - push logs updates to Telegram [![crates.io](https://img.shields.io/crates/v/logram.svg)](https://crates.io/crates/logram) [![travis-ci.org](https://api.travis-ci.org/Ralvke/logram.svg?branch=master)](https://travis-ci.org/Ralvke/logram)

## Use
1. Install Rust via [Rustup](https://rustup.rs/)
2. Install logram: `cargo install logram`
3. Create bot
4. Run logram in echoID mode: `logram echoID <bot token>` and get needed chat ids:
    * For regular chat: send any message to bot
    * For group chat: add bot to group
    * For channel: forward any message from channel to bot (dont forget add bot to channel admins)
5. Write config from example
6. Run in normal mode `logram <path to config>`

## Config example
```yaml
telegram:
  # bot token
  token: 123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11
  # default chat (for updates from files without custom chat and errors)
  chat: 12345678 

watcher:
  # watching files (logs)
  files: 
      # path to file
    - path: /var/log/awesome.log 
      # custom chat for this file (optional)
      chat: 12345678 
      # push updates only which matches that regex (optional) 
      # (more info: doc.rust-lang.org/regex)
      regex: critical 
```

 ## Systemd service
 1. Create link: `sudo ln -s /home/<user>/.cargo/bin/logram /usr/bin/logram`
 2. Copy config to `/etc/logram.yaml`
 3. Copy `logram.service` to `/etc/systemd/system`
 4. Reload services: `systemctl daemon-reload`
 5. Enable service: `sudo systemctl enable logram`
 6. Run service: `sudo systemctl start logram`