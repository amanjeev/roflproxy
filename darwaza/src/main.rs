#![warn(rust_2018_idioms)]

use {
    std::{
        collections::HashMap,
        net::SocketAddr,
    },
};

use clap::{App, Arg, SubCommand};

use {
    hyper::{
        Body, Client, Request, Response, Server, Uri,

        // This function turns a closure which returns a future into an
        // implementation of the the Hyper `Service` trait, which is an
        // asynchronous function from a generic `Request` to a `Response`.
        service::service_fn,

        // A function which runs a future to completion using the Hyper runtime.
        rt::run,
    },
    futures::{
        // Extension trait for futures 0.1 futures, adding the `.compat()` method
        // which allows us to use `.await` on 0.1 futures.
        compat::Future01CompatExt,
        // Extension traits providing additional methods on futures.
        // `FutureExt` adds methods that work for all futures, whereas
        // `TryFutureExt` adds methods to futures that return `Result` types.
        future::{FutureExt, TryFutureExt},
    },
};

async fn server_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let url = "https://www.rust-lang.org/en-US/";
    let url = url.parse::<Uri>().expect("failed to parse url");
    let res = Client::new().get(url).compat().await;
    println!("request finished-- returning response");
    res
}

async fn run_proxy_server(addr: SocketAddr) {
    println!("Listening on address: {}", addr);
    let serve_future = Server::bind(&addr)
        .serve(|| service_fn(|req| server_request(req).boxed().compat()));

    if let Err(e) = serve_future.compat().await {
        eprintln!("server error: {}", e);
    }
}

fn main() {
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

    // start proxy server
    let listener_addr = format!("{}:{}", settings_hash["listening_host"], settings_hash["listening_port"]);
    let listener_addr: SocketAddr = listener_addr.parse().expect("unable to parse socket address");

    let futures_03_future = run_proxy_server(listener_addr);
    let futures_01_future = futures_03_future.unit_error().boxed().compat();

    run(futures_01_future);
}










