use std::{thread, time::Duration};

use crate::source::LogRecord;

use super::{Debounce, Debouncer};

#[test]
fn test() {
    let mut debouncer = Debouncer::new(1);

    let record_a1 = LogRecord {
        title: String::from("a"),
        body: String::from("one"),
    };
    let record_a1_debounce = Debounce::NewMessage(&record_a1);
    assert_eq!(debouncer.debounce(&record_a1), record_a1_debounce);
    debouncer.on_message_sent(record_a1, 1);

    let record_a2 = LogRecord {
        title: String::from("a"),
        body: String::from("two"),
    };
    let record_a2_debounce = Debounce::EditMessage {
        id: 1,
        title: String::from("a"),
        body: vec![String::from("one"), String::from("two")],
    };
    assert_eq!(debouncer.debounce(&record_a2), record_a2_debounce);
    debouncer.on_message_sent(record_a2, 1);

    let record_b1 = LogRecord {
        title: String::from("b"),
        body: String::from("one"),
    };
    let record_b1_debounce = Debounce::NewMessage(&record_b1);
    assert_eq!(debouncer.debounce(&record_b1), record_b1_debounce);

    thread::sleep(Duration::from_secs(1));

    let record_b2 = LogRecord {
        title: String::from("b"),
        body: String::from("two"),
    };
    let record_b2_debounce = Debounce::NewMessage(&record_b2);
    assert_eq!(debouncer.debounce(&record_b2), record_b2_debounce);
}
