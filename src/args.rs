use clap::{App, AppSettings, Arg, SubCommand};

pub fn clap_app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .help("Path to config file")
                .value_name("FILE")
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("echo_id")
                .about("Run in EchoID mode")
                .arg(
                    Arg::with_name("token")
                        .long("token")
                        .short("t")
                        .help("Telegram bot token")
                        .value_name("TOKEN")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("proxy")
                        .long("proxy")
                        .short("p")
                        .help("Proxy for requests")
                        .value_name("URL")
                        .takes_value(true),
                ),
        )
}
