# logram - push logs updates to Telegram

## Use
1. Install Rust via [Rustup](https://rustup.rs/)
2. Install logram: `cargo install logram`
3. Create bot
3. Write config without `telegram.chat`
4. Get chat id and put it to config
4. Run `logram <path to config>`

## Getting chat id
* For regular chat: send any message to bot - bot will respond chat id
* For group chat: add bot to group - bot will respond chat id to group
* For channel: forward any message from channel to bot - bot will respond chat id (dont forget add bot to channel's admins)

## Config example
```yaml
telegram:
  token: 123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11 # bot token
  chat: 12345678 # default chat
```
