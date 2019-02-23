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

        for unit in config.units {
            journal.match_add("_SYSTEMD_UNIT", unit)?;
            journal.match_or()?;
        }

        Ok(JournaldLogSource { journal })
    }
}

// www.freedesktop.org/software/systemd/man/sd-journal.html#Thread%20safety
unsafe impl Send for JournaldLogSource {}

impl LogSource for JournaldLogSource {
    fn into_stream(mut self) -> Box<LogSourceStream> {
        let (tx, rx) = futures_mpsc::unbounded();
        let tx_clone = tx.clone();

        let on_event = move |event| {
            let event = JournaldEvent::from(event);
            let event = LogSourceEvent::Record(event.into());
            tx.unbounded_send(event).unwrap();

            Ok(())
        };

        thread::spawn(move || {
            if let Err(error) = self.journal.watch_all_elements(on_event) {
                let error = LogSourceEvent::Error(Error::from(error));
                tx_clone.unbounded_send(error).unwrap();
            }
        });

        Box::new(rx)
    }
}
