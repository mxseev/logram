extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate reqwest;
extern crate url;

use std::process;

mod error;
mod config;
mod telegram;
use error::InitError;
use config::Config;
use telegram::Telegram;


fn init() -> Result<(), InitError> {
    let config = Config::read()?;
    let mut telegram = Telegram::new(config.telegram)?;

    telegram.listen_updates()?;
    Ok(())
}

fn main() {
    if let Err(e) = init() {
        println!("init error: {}", e);
        process::exit(2);
    }
}
