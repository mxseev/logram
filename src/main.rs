use clap::{load_yaml, App};
use failure::Error;
use std::process;

mod config;
use self::config::Config;

fn run() -> Result<(), Error> {
    let cli = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(cli).get_matches();

    let config_filename = matches.value_of("config").unwrap_or("config.yaml");
    let config = Config::read(config_filename)?;

    println!("{:?}", config);

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        println!("Error: {}", error);
        process::exit(2);
    }
}
