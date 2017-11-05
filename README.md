# logram - push logs updates to Telegram

## Use
1. Install Rust via [Rustup](https://rustup.rs/)
2. Install logram: `cargo install logram`
3. Create bot
4. Run logram in echoID mode: `logram echoID <bot token>` and get needed chat ids:
  * For regular chat: send any message to bot
  * For group chat: add bot to group
  * For channel: forward any message from channel to bot (dont forget add bot to channel's admins)
4. Write config from example
5. Run in normal mode `logram <path to config>`

## Config example
```yaml
telegram:
  token: 123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11 # bot token
  chat: 12345678 # default chat

watcher:
  files: # watching files (logs)
    - path: /var/log/awesome.log # path to file
      chat: 12345678 # custom chat for this file (optional)
      regex: critical # push updates only which matches that regex (optional) (more info: doc.rust-lang.org/regex)
```
