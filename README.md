# logram - push logs updates to Telegram

## Use
1. Install Rust via [Rustup](https://rustup.rs/)
2. Install logram: `cargo install logram`
3. Create bot
4. Run logram in echoID mode: `logram echoID <bot token>` and get needed chat ids:
  * For regular chat: send any message to bot
  * For group chat: add bot to group
  * For channel: forward any message from channel to bot (dont forget add bot to channel admins)
4. Write config from example
5. Run in normal mode `logram <path to config>`

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
