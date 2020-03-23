use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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

#[derive(Clone, Debug, Default)]
pub struct RoflRouteConfig {}
