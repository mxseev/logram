extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::process;

mod error;
mod config;
use error::InitError;
use config::Config;


fn main() {
    if let Err(e) = init() {
        println!("init error: {}", e);
        process::exit(2);
    }
}

fn init() -> Result<(), InitError> {
    let config = Config::read()?;
    println!("{:?}", config);

    Ok(())
}
