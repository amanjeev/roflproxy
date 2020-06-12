use clap::{App, ArgMatches};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;

struct PathDef<'s>(&'s Path);

/// Type to hold the server's own config
pub struct ServerConfig<'s, 'c> {
    pub addr: SocketAddr, // ip:port
    pub config: &'c PathDef<'s>,
}

impl<'s, 'c> ServerConfig<'s, 'c> {
    pub fn new() -> Self {
        let initial = Self::init_config();

        let addr = match initial.value_of("address") {
            Some(a) => a.parse().unwrap(),
            _ => "127.0.0.1:12666".parse().unwrap(),
        };

        let config = match initial.value_of("config") {
            Some(c) => Path::new(c),
            _ => Path::new("/tmp/rofl"),
        };

        ServerConfig { addr, config }
    }

    fn init_config() -> ArgMatches {
        let matches = App::new("darwaza")
            .about("The gateway")
            .arg("-a, --address=[ADDRESS:PORT] 'The address:port on which proxy listens'")
            .arg("-c, --config=[FILE] 'Sets the API config file'");

        matches.get_matches()
    }
}

impl<'s, 'c> Default for ServerConfig<'s, 'c> {
    fn default() -> Self {
        ServerConfig {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12666),
            config: Path::new("/tmp/rofl"),
        }
    }
}

impl<'s, 'c> Clone for ServerConfig<'s, 'c> {
    fn clone(&self) -> Self {
        ServerConfig {
            addr: self.addr,
            config: self.config,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteInfo {
    addr: SocketAddr,
    methods: Vec<String>,
    //TODO: add local settings here like
    // timeouts, retries etc.
}

/// Type to hold the router config to
/// route traffic to downstream servers
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RouterConfig {
    pub routemap: HashMap<String, RouteInfo>,
    //TODO: add global settings here like
    // timeouts, retries etc.
}

impl RouterConfig {
    pub fn from_file(file_path: &Path) -> Self {
        let contents = fs::read_to_string(file_path).expect("Error while trying to read the file");

        serde_yaml::from_str(contents.as_str().as_ref()).expect("Error while trying to serialize")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::method::Method;
    use serde_yaml;
    use std::env::current_dir;

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
            config: Path::new("/tmp/rofl"),
        };
        assert_eq!(server_config.addr.port(), 8080);
        assert!(server_config.addr.ip().is_ipv4());
        assert_eq!(server_config.addr.to_string().as_str(), "10.10.0.1:8080");
    }

    #[test]
    fn rofl_router_config_stuff() {
        let mut routes = HashMap::new();
        routes.insert(
            "/about".to_string(),
            RouteInfo {
                addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)), 8080),
                methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string()],
            },
        );
        routes.insert(
            "/blog".to_string(),
            RouteInfo {
                addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)), 8081),
                methods: vec!["GET".to_string(), "HEAD".to_string()],
            },
        );
        routes.insert(
            "/contact".to_string(),
            RouteInfo {
                addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 2)), 8083),
                methods: vec![
                    Method::GET.to_string(),
                    Method::DELETE.to_string(),
                    Method::POST.to_string(),
                    Method::HEAD.to_string(),
                    Method::PUT.to_string(),
                    Method::OPTIONS.to_string(),
                    Method::PATCH.to_string(),
                ],
            },
        );
        let router_config = RouterConfig { routemap: routes };
        let serialized_config = serde_yaml::to_string(&router_config).unwrap();
        println!("{}", serialized_config);

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
            .contains(&Method::PATCH.to_string()));
    }

    #[test]
    fn router_config_file() {
        let mut current_directory = current_dir()
            .expect("Failed to fetch the current directory")
            .to_str()
            .expect("Failed to convert current directory PathBuf to str")
            .to_string();

        current_directory.push_str("/src/routertest.yaml");
        let conf = RouterConfig::from_file(Path::new(current_directory.as_str()));

        assert_eq!(
            conf.routemap["/about"].addr.to_string().as_str(),
            "10.10.0.1:8080"
        );
    }
}
