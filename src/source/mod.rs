use failure::Error;
use futures::Stream;
use std::fmt::Debug;

mod fs;
mod journald;
pub use self::{fs::FsLogSource, journald::JournaldLogSource};

pub trait LogRecord: Debug + Send {
    fn to_message(&self) -> String;
}

#[derive(Debug)]
pub enum LogSourceEvent {
    Record(Box<LogRecord>),
    Error(Error),
}
impl LogSourceEvent {
    pub fn to_message(&self) -> String {
        match self {
            LogSourceEvent::Record(record) => record.to_message(),
            LogSourceEvent::Error(error) => format!("Error: {}", error),
        }
    }
}

pub type LogSourceStream = Stream<Item = LogSourceEvent, Error = ()> + Send;

pub trait LogSource {
    fn into_stream(self) -> Box<LogSourceStream>;
}
