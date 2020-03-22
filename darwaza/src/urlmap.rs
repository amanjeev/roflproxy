use std::collections::HashMap;
use std::error::Error;
use tide::Request as RequestAsServer;
use url::Url;

/// Trait to extend functionality of Url crate
/// with Url mapping using a mapping file
pub trait UrlMap {
    /// Returns HashMap of the endpoints to target server mapping
    fn map_uri_to_target() -> HashMap<&'static str, &'static str> {
        // TODO: get this map from config
        let mut uri_to_target_map: HashMap<&str, &str> = HashMap::new();
        uri_to_target_map.insert("/home", "http://127.0.0.1:8000");
        uri_to_target_map.insert("/about", "http://127.0.0.1:8000");

        uri_to_target_map
    }

    /// Returns a target server URL given the `request` object
    /// If the mapping is
    ///     ```
    ///     "/home"  => "foo.com:1234"
    ///     "/staff" => "bar.org:4567"
    ///     ```
    /// and the request is for
    ///     `/staff/jdoe`
    /// then, the target server is `foo.com:1234/jdoe`
    fn get_target_server_url(request: &RequestAsServer<()>) -> Result<Url, Box<dyn Error>> {
        let incoming_request_uri = request.uri().clone();
        // get the target from uri segment using the map
        let uri_map = Self::map_uri_to_target();
        let mut result_server = "".to_string();
        let mut result_url = "".to_string();
        dbg!(incoming_request_uri.clone());
        for (uri, server) in uri_map.iter() {
            if incoming_request_uri
                .to_string()
                .as_str()
                .starts_with(uri.to_string().as_str())
            {
                dbg!("Matched: ", uri.clone());
                result_server = server.to_string();
                result_url = incoming_request_uri.to_string().replace(uri, "");
                dbg!(result_server.clone(), result_url.clone());
            } else {
                dbg!("No match for: ", uri.clone());
            }
        }
        let result_target = Url::parse(result_server.as_str())?.join(result_url.as_str())?;

        dbg!("The final Target>> ", result_target.clone());

        Ok(result_target)
    }
}

impl UrlMap for Url {}
