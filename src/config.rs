use failure::Error;
use serde_derive::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Config {
    some: bool,
}

impl Config {
    pub fn read(filename: &str) -> Result<Self, Error> {
        let file = File::open(filename)?;
        let config: Self = serde_yaml::from_reader(file)?;

        Ok(config)
    }
}
