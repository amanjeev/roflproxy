use clap::{App, ArgMatches};
use http::method::Method;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Type to hold the server's own config
pub struct ServerConfig {
    pub addr: SocketAddr, // ip:port
}

impl ServerConfig {
    pub fn new() -> Self {
        let initial = Self::init_config();
        let addr = match initial.value_of("address") {
            Some(a) => a.parse().unwrap(),
            _ => "127.0.0.1:12666".parse().unwrap(),
        };

        ServerConfig { addr }
    }

    fn init_config() -> ArgMatches {
        let matches = App::new("darwaza")
            .about("The gateway")
            .arg("-a, --address=[ADDRESS:PORT] 'The address:port on which proxy listens'")
            .arg("-c, --config=[FILE] 'Sets the API config file'");

        matches.get_matches()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12666),
        }
    }
}

impl Clone for ServerConfig {
    fn clone(&self) -> Self {
        ServerConfig { addr: self.addr }
    }
}

#[derive(Clone, Debug)]
pub struct RouteInfo {
    addr: SocketAddr,
    methods: Vec<Method>,
    //TODO: add local settings here like
    // timeouts, retries etc.
}

/// Type to hold the router config to
/// route traffic to downstream servers
#[derive(Clone, Debug, Default)]
pub struct RouterConfig {
    pub routemap: HashMap<&'static str, RouteInfo>,
    //TODO: add global settings here like
    // timeouts, retries etc.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rofl_server_config_default() {
        let server_config = ServerConfig::default();
        assert_eq!(server_config.addr.port(), 12666);
        assert!(server_config.addr.ip().is_ipv4());
        assert_eq!(server_config.addr.to_string().as_str(), "127.0.0.1:12666");
    }

    #[test]
    fn rofl_server_config_stuff() {
        let server_config = ServerConfig {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)), 8080),
        };
        assert_eq!(server_config.addr.port(), 8080);
        assert!(server_config.addr.ip().is_ipv4());
        assert_eq!(server_config.addr.to_string().as_str(), "10.10.0.1:8080");
    }

    #[test]
    fn rofl_router_config_stuff() {
        let mut routes = HashMap::new();
        routes.insert(
            "/about",
            RouteInfo {
                addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)), 8080),
                methods: vec![Method::GET, Method::POST, Method::PUT],
            },
        );
        routes.insert(
            "/blog",
            RouteInfo {
                addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)), 8081),
                methods: vec![Method::GET, Method::HEAD],
            },
        );
        routes.insert(
            "/contact",
            RouteInfo {
                addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 2)), 8083),
                methods: vec![
                    Method::GET,
                    Method::DELETE,
                    Method::POST,
                    Method::HEAD,
                    Method::PUT,
                    Method::OPTIONS,
                    Method::PATCH,
                ],
            },
        );
        let router_config = RouterConfig { routemap: routes };

        assert_eq!(
            router_config.routemap["/about"].addr,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)), 8080)
        );
        assert_eq!(
            router_config.routemap["/blog"].addr.to_string().as_str(),
            "10.10.0.1:8081"
        );
        assert_eq!(router_config.routemap["/contact"].addr.port(), 8083);
        assert_eq!(router_config.routemap["/contact"].methods.len(), 7);
        assert!(router_config.routemap["/contact"]
            .methods
            .contains(&Method::PATCH));
    }
}
