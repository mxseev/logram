use std::io;
use std::fmt;
use std::error::Error;
use serde_yaml;


#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Yaml(serde_yaml::Error),
}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cause = match self.cause() {
            Some(e) => format!(" ({})", e),
            None => String::new(),
        };
        write!(f, "{}{}", self.description(), &cause)
    }
}
impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Io(_) => "Could not read config file",
            ConfigError::Yaml(_) => "Could not deserialize config file",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match self {
            &ConfigError::Io(ref e) => Some(e),
            &ConfigError::Yaml(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}
impl From<serde_yaml::Error> for ConfigError {
    fn from(e: serde_yaml::Error) -> Self {
        ConfigError::Yaml(e)
    }
}
