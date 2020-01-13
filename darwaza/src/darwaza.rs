#![warn(rust_2018_idioms)]

use std::collections::HashMap;

use clap::{App, Arg, SubCommand};
use tokio::io;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let matches = App::new("darwaza")
        .about("Reverse Proxy")
        .subcommand(SubCommand::with_name("start"))
        .about("Run the proxy")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .help("Sets custom file")
            .value_name("CONFIG_FILE")
            .takes_value(true)
            .required(option_env!("DARWAZA_CONFIG").is_none()))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("settings.toml");

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(config)).unwrap();
    let settings_hash = settings.try_into::<HashMap<String, String>>().unwrap();
    println!("{:?}", settings_hash);
}
