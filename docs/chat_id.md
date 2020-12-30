# Determining chat ID

The logram needs a Telegram chat id to send messages. To determine it, you can use logram's echo-id mode. In this mode, logram will report (both in Telegram and in the console) the id of the chats. Just run logram in this mode e.g. `logram echo_id -t <bot token>`, and:
- For private chats send to bot any message
- For group chats add the bot to this chat
- For channels forward a message from this channel to the bot (and don't forget to add the bot to the channel administrators)

```sh
$ logram echo_id -t 123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11
The ID of chat with @seevmx: 79098882
The chat ID of group "test group": -49301488
The chat ID of channel "test channel": -10018818605
```

Run `logram echo_id --help` for see a full list of available options. Alternatively, you can use the [@RawDataBot](https://t.me/RawDataBot).
