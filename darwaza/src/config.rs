use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Type to hold the server's own config
pub struct RoflServerConfig {
    pub addr: SocketAddr, // ip:port
}

impl Default for RoflServerConfig {
    fn default() -> Self {
        RoflServerConfig {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12666),
        }
    }
}

impl Clone for RoflServerConfig {
    fn clone(&self) -> Self {
        RoflServerConfig { addr: self.addr }
    }
}

/// Type to hold the router config to
/// route traffic to downstream servers
#[derive(Clone, Debug, Default)]
pub struct RoflRouterConfig {
    pub routermap: HashMap<&'static str, SocketAddr>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rofl_server_config_default() {
        let server_config = RoflServerConfig::default();
        assert_eq!(server_config.addr.port(), 12666);
        assert!(server_config.addr.ip().is_ipv4());
        assert_eq!(server_config.addr.to_string().as_str(), "127.0.0.1:12666");
    }

    #[test]
    fn rofl_server_config_stuff() {
        let server_config = RoflServerConfig {
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
        let router_config = RoflRouterConfig { routermap: routes };
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
