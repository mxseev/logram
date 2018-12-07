use failure::Error;
use futures::sync::mpsc as futures_mpsc;
use std::thread;
use systemd::journal::{Journal, JournalFiles, JournalSeek};

use crate::{
    config::JournaldLogSourceConfig,
    source::{LogSource, LogSourceEvent, LogSourceStream},
};

mod event;
use self::event::JournaldEvent;

pub struct JournaldLogSource {
    journal: Journal,
}

impl JournaldLogSource {
    pub fn new(config: JournaldLogSourceConfig) -> Result<Self, Error> {
        let mut journal = Journal::open(JournalFiles::All, false, true)?;
        journal.seek(JournalSeek::Tail)?;

        for service in config.services {
            journal.match_add("_SYSTEMD_UNIT", service)?;
            journal.match_or()?;
        }

        Ok(JournaldLogSource { journal })
    }
    fn next_event(&mut self) -> Result<JournaldEvent, Error> {
        if let Some(event) = self.journal.await_next_record(None)? {
            return Ok(JournaldEvent::from(event));
        }

        self.next_event()
    }
}

// www.freedesktop.org/software/systemd/man/sd-journal.html#Thread%20safety
unsafe impl Send for JournaldLogSource {}

impl LogSource for JournaldLogSource {
    fn into_stream(self) -> Box<LogSourceStream> {
        let (mut tx, rx) = futures_mpsc::channel(10);

        thread::spawn(move || {
            let mut source = self;

            loop {
                let event = match source.next_event() {
                    Ok(event) => LogSourceEvent::Record(Box::new(event)),
                    Err(error) => LogSourceEvent::Error(error),
                };

                tx.try_send(event).unwrap();
            }
        });

        Box::new(rx)
    }
}
