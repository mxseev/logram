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
    if env::args().nth(1) == Some("echoID".to_string()) {
        let token = match env::args().nth(2) {
            Some(token) => token,
            None => {
                println!("Usage: logram echoID <bot token>");
                process::exit(1)
            }
        };
        println!("Running in echoID mode, chat ids will be printed here..");
        Telegram::echo_id(token)?;
    }

    let config = Config::read()?;
    let telegram = Telegram::new(config.telegram.into())?;
    let mut watcher = FileWatcher::new(config.watcher, telegram.clone())?;

    telegram.send(Message {
        chat: None,
        body: MessageBody::Started,
    })?;

    if let Err(e) = watcher.watch_files() {
        let message = Message {
            chat: None,
            body: MessageBody::Error { content: format!("{}", e) },
        };
        telegram.send(message)?;
        return Err(InitError::from(e));
    }

    Ok(())
}

fn main() {
    if let Err(e) = init() {
        println!("error: {}", e);
        process::exit(2);
    }
}
