extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate reqwest;
extern crate url;
extern crate notify;
extern crate regex;

use std::{process, env};

mod error;
mod config;
mod telegram;
mod watcher;
use error::InitError;
use config::Config;
use telegram::{Telegram, Message, MessageBody};
use watcher::FileWatcher;


fn init() -> Result<(), InitError> {
    if let Some(first_arg) = env::args().nth(1) {
        if first_arg == String::from("echoID") {
            if let Some(token) = env::args().nth(2) {
                println!("Running in echoID mode, chat ids will be printed here");
                Telegram::echo_id(token)?;
            } else {
                println!("Usage: logram echoID <bot token>");
                process::exit(1);
            }
        }
    }

    let config = Config::read()?;
    let telegram = Telegram::new(config.telegram)?;
    let mut watcher = FileWatcher::new(config.watcher, telegram.clone())?;

    match watcher.watch_files() {
        Ok(_) => Ok(()),
        Err(e) => {
            let message = Message {
                chat: None,
                body: MessageBody::Error { content: format!("{}", e) },
            };
            telegram.send(message)?;
            Err(InitError::from(e))
        }
    }
}

fn main() {
    if let Err(e) = init() {
        println!("error: {}", e);
        process::exit(2);
    }
}
