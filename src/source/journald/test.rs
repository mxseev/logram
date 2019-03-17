use futures::Stream;
use systemd::journal;

use crate::{
    config::JournaldLogSourceConfig,
    source::{LogRecord, LogSource},
};

use super::JournaldLogSource;

fn send_log_record(keys: Vec<(&str, &str)>) {
    let keys: Vec<String> = keys
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect();

    let keys = keys.iter().map(AsRef::as_ref).collect::<Vec<_>>();

    journal::send(&keys);
}

fn record(title: &str, body: &str) -> LogRecord {
    LogRecord {
        title: String::from(title),
        body: String::from(body),
    }
}

#[test]
fn test() {
    let config = JournaldLogSourceConfig {
        units: vec![String::from("user@1000.service")],
    };

    let source = JournaldLogSource::new(config).unwrap();
    let mut stream = source.into_stream().wait();

    send_log_record(vec![("MESSAGE", "logram test")]);
    assert_eq!(
        stream.next().unwrap().unwrap(),
        record("user@1000.service", "logram test")
    );

    send_log_record(vec![("MESSAGE", "logram test 2")]);
    assert_eq!(
        stream.next().unwrap().unwrap(),
        record("user@1000.service", "logram test 2")
    );
}
