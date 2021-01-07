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

## Journald
Allows you to retrieve entries from the journald. Configurable with filters that match journald record entries. To view raw journald entries you can use `journalctl -f -o json-pretty`.

```yaml
journald:
  enabled: true
  matches:
    - title: Kernel message # Title for telegram message
      filters:
        _TRANSPORT: kernel

    - title: CUPS service
      filters:
        _SYSTEMD_UNIT: cups.service

    - title: Audit sudo usage
      filters:
        _TRANSPORT: audit
        AUDIT_FIELD_EXE: /usr/bin/sudo
```

## Docker
Reads the logs from the docker.

```yaml
docker:
  enabled: true
  transport: local # connecting transport, supported values "local", "unix" and "http", default local
  addr: "unix:///var/run/docker.sock" # address for connecting, default "unix:///var/run/docker.sock"
  timeout: 10 # timeout of connecting, default 120
```
