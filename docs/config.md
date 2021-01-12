# Configuration

Logram is configured using a .yaml file located at `/etc/logram.yaml`. The schematic of this file:

```yaml
hello_message: true # send a greeting message at startup, default true

telegram:
  token: 123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11 # Telegram bot token
  chat_id: 12345678 # Telegram chat ID (see docs/chat_id.md for details)

sources: # log sources config (see docs/log_sources.md for details)
  counter:
    enabled: true # enables or disables the log source, each log source has this setting, default false
    
  filesystem:
    enabled: true
    entries:
      - /var/log/nginx

  journald:
    enabled: true
    matches:
      - title: CUPS service
        filters:
          _SYSTEMD_UNIT: cups.service

  docker:
    enabled: false
```
