use failure::Error;
use std::thread;
use systemd::journal::{Journal, JournalFiles, JournalSeek};

use crate::{
    config::JournaldLogSourceConfig,
    source::{LogSource, LogSourceStream},
    utils::result_channel,
};

mod record;
#[cfg(test)]
mod test;
use self::record::map_record;

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
        let (tx, rx) = result_channel();
        let tx_clone = tx.clone();

        let on_record = move |record| {
            let record = map_record(record);
            tx.unbounded_send(Ok(record)).unwrap();

            Ok(())
        };

        let thread_task = move || {
            if let Err(error) = self.journal.watch_all_elements(on_record) {
                let error = Error::from(error);
                tx_clone.unbounded_send(Err(error)).unwrap();
            }
        };

        thread::spawn(thread_task);

        Box::new(rx)
    }
}
