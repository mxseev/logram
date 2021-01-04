#![recursion_limit = "128"]

use clap::{crate_version, load_yaml, App};
use failure::Error;
use futures::{stream, Future, Stream};
use std::process;
use tgbot::{methods::SendMessage, types::ParseMode, Api as TelegramApi};
use tokio;

mod config;
mod echo_id;
mod source;
mod utils;
use self::{
    config::Config,
    echo_id::echo_id,
    source::{FsLogSource, LogSource},
};
#[cfg(feature = "journald-source")]
use self::source::JournaldLogSource;

fn run() -> Result<(), Error> {
    let cli = load_yaml!("../cli.yaml");
    let app = App::from_yaml(cli).version(crate_version!());
    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("echo_id") {
        let token = matches.value_of("token").unwrap();
        return echo_id(token);
    }

    let config_filename = matches.value_of("config").unwrap_or("config.yaml");
    let config = Config::read(config_filename)?;

    let telegram = TelegramApi::new::<String, String>(config.telegram.token, None)?;
    let chat_id = config.telegram.chat_id.clone();

    let fs = FsLogSource::new(config.sources.fs)?;
    let fs_stream = fs.into_stream();

    #[cfg(feature = "journald-source")]
    let journald_stream = JournaldLogSource::new(config.sources.journald)?
        .into_stream();

    let used_source_streams = {
        let s = stream::empty();
        let s = s.select(fs_stream);

        #[cfg(feature = "journald-source")]
        let s = s.select(journald_stream);
        s
    };

    let main_loop = used_source_streams.then(move |result| {
                let text = match result {
                    Ok(record) => format!("*{}*```\n{}```", record.title, record.body),
                    Err(error) => format!("Error: {}", error),
                };

                Ok(text)
            })
            .map(move |text| SendMessage::new(chat_id.as_str(), text).parse_mode(ParseMode::Markdown))
            .for_each(move |method| telegram.execute(&method).map(|_| ()))
            .map_err(|error| eprintln!("Telegram error: {}", error));

    tokio::run(main_loop);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        process::exit(2);
    }
}
