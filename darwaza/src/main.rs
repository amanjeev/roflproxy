#![warn(rust_2018_idioms)]

use std::collections::HashMap;

use clap::{App, Arg, SubCommand};

use futures::future::try_join;
use futures::stream::StreamExt;

use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;


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

    // server
    let listener_addr = format!("{}:{}", settings_hash["listening_host"], settings_hash["listening_port"]);
    let mut listener = TcpListener::bind(listener_addr.clone()).await.unwrap();

    let proxy_server = async move {
        let mut incoming = listener.incoming();

        while let Some(conn) = incoming.next().await {
            match conn {
                Err(err) => { println!("accept error = {:?}", err); }
                Ok(mut socket) => {
                    println!("Accepted connection from {:?}", socket.peer_addr());
                    tokio::spawn(async move {
                        // spawn a transfer task here
                        let tramsfer = transfer()
                    });
                }
            }
        }
    };

    println!("Proxy server running on {}", listener_addr);
    proxy_server.await;

    // start client here to connect to a running server
    // you can run a server by
    // `socat TCP-LISTEN:6142,fork stdout`
    /*let target = format!("{}:{}", settings_hash["listening_host"], settings_hash["listening_port"]);
    let mut stream = TcpStream::connect(target).await.unwrap();
    println!("Created stream");

    let result = stream.write(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());*/
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) {
    let mut outbound = TcpStream::connect(proxy_addr).await.unwrap();

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = io::copy(&mut ri, &mut wo);
    let server_to_client = io::copy(&mut ro, &mut wi);

    try_join(client_to_server, server_to_client).await.unwrap();
}

















