# Supported log sources

## Counter
Just sends a message at a defined interval. Created only for testing purpose.

```yaml
counter:
  enabled: true
  interval: 1000 # interval in ms between messages, default 10000
  initial: 42 # initial value of counter, default 1
```
