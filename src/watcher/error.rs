use std::fmt;
use std::error::Error;
use std::io;
use std::sync::mpsc;
use notify;

use telegram::TelegramError;


#[derive(Debug)]
pub enum WatcherError {
    Io(io::Error),
    DirsNotSupported,
    ParentDirNotFound,
    Notify(notify::Error),
    Recv(mpsc::RecvError),
    Telegram(TelegramError),
}
impl fmt::Display for WatcherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cause = match self.cause() {
            Some(e) => format!(" ({})", e),
            None => String::new(),
        };
        write!(f, "{}{}", self.description(), &cause)
    }
}
impl Error for WatcherError {
    fn description(&self) -> &str {
        match *self {
            WatcherError::Io(_) => "Could not read file",
            WatcherError::DirsNotSupported => "Only files watching supported",
            WatcherError::ParentDirNotFound => "Parent dir not found",
            WatcherError::Notify(_) => "FS watcher error",
            WatcherError::Recv(_) => "FS watcher channel error",
            WatcherError::Telegram(_) => "Telegram error",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match self {
            &WatcherError::Io(ref e) => Some(e),
            &WatcherError::DirsNotSupported => None,
            &WatcherError::ParentDirNotFound => None,
            &WatcherError::Notify(ref e) => Some(e),
            &WatcherError::Recv(ref e) => Some(e),
            &WatcherError::Telegram(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for WatcherError {
    fn from(e: io::Error) -> Self {
        WatcherError::Io(e)
    }
}
impl From<notify::Error> for WatcherError {
    fn from(e: notify::Error) -> Self {
        WatcherError::Notify(e)
    }
}
impl From<mpsc::RecvError> for WatcherError {
    fn from(e: mpsc::RecvError) -> Self {
        WatcherError::Recv(e)
    }
}
impl From<TelegramError> for WatcherError {
    fn from(e: TelegramError) -> Self {
        WatcherError::Telegram(e)
    }
}
