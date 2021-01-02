# Supported log sources

## Counter
Just sends a message at a defined interval. Created only for testing purpose.

```yaml
counter:
  enabled: true
  interval: 1000 # interval in ms between messages, default 10000
  initial: 42 # initial value of counter, default 1
```

## Filesystem
Gets records from log files. Supports files and folders (recursively).

```yaml
filesystem:
  enabled: true
  delay: 1000 # delay for event's debounce, default 1000
  entries: # paths to watching files/dirs
    - /var/log/nginx
    - /var/log/cups/error_log
```
