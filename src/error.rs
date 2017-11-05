use std::fmt;
use std::error::Error;

use config::ConfigError;
use telegram::TelegramError;
use watcher::WatcherError;


#[derive(Debug)]
pub enum InitError {
    Config(ConfigError),
    Telegram(TelegramError),
    Watcher(WatcherError),
}
impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cause = match self.cause() {
            Some(e) => format!("{}", e),
            None => String::new(),
        };
        write!(f, "{}: {}", self.description(), &cause)
    }
}
impl Error for InitError {
    fn description(&self) -> &str {
        match *self {
            InitError::Config(_) => "Config",
            InitError::Telegram(_) => "Telegram",
            InitError::Watcher(_) => "Watcher",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            InitError::Config(ref e) => Some(e),
            InitError::Telegram(ref e) => Some(e),
            InitError::Watcher(ref e) => Some(e),
        }
    }
}

impl From<ConfigError> for InitError {
    fn from(e: ConfigError) -> Self {
        InitError::Config(e)
    }
}
impl From<TelegramError> for InitError {
    fn from(e: TelegramError) -> Self {
        InitError::Telegram(e)
    }
}
impl From<WatcherError> for InitError {
    fn from(e: WatcherError) -> Self {
        InitError::Watcher(e)
    }
}
