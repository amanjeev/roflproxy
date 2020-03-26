use clap::{App, ArgMatches};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};

/// Type to hold the server's own config
pub struct ServerConfig {
    pub addr: SocketAddr, // ip:port
}

impl ServerConfig {
    pub fn new() -> Self {
        let initial = Self::init_config();
        let addr = match initial.value_of("config") {
            Some(a) => a.parse().unwrap(),
            _ => "127.0.0.1:12666".parse().unwrap(),
        };

        ServerConfig { addr }
    }

    fn init_config() -> ArgMatches {
        let matches = App::new("darwaza")
            .about("The gateway")
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

/// Type to hold the router config to
/// route traffic to downstream servers
#[derive(Clone, Debug, Default)]
pub struct RouterConfig {
    pub routermap: HashMap<&'static str, SocketAddr>,
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
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)), 8080),
        );
        routes.insert(
            "/blog",
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)), 8081),
        );
        routes.insert(
            "/contact",
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 2)), 8083),
        );
        let router_config = RouterConfig { routermap: routes };
        assert_eq!(
            router_config.routermap["/about"],
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)), 8080)
        );
        assert_eq!(
            router_config.routermap["/blog"].to_string().as_str(),
            "10.10.0.1:8081"
        );
        assert_eq!(router_config.routermap["/contact"].port(), 8083);
    }
}
