use std::collections::HashMap;
use std::error::Error;
use surf::{Request as RequestAsClient, Response as ResponseAsClient};
use tide::{Request as RequestAsServer, Response as ResponseAsServer};
use url::Url;

pub struct Proxificate;

impl Proxificate {
    /// Receives request from client, returns corresponding request object
    /// to the downstream server
    pub async fn map_request(
        mut request_from_client: RequestAsServer<()>,
    ) -> RequestAsClient<http_client::isahc::IsahcClient> {
        let method = request_from_client.method().clone();

        let target_server_url = Self::target(request_from_client)
            .await
            .unwrap_or(Url::parse("http://127.0.0.1:8000/404.html").unwrap());
        let request_for_downstream = RequestAsClient::new(method, target_server_url);

        request_for_downstream
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
    async fn target(request: RequestAsServer<()>) -> Result<Url, Box<dyn Error>> {
        let incoming_request_uri = request.uri().clone();
        // get the target from uri segment using the map
        let uri_map = Self::map_uri_to_target().await;
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

    /// Receives response from downstream server, returns corresponding
    /// response object to the client
    pub fn map_response(response_from_downstream: ResponseAsClient) -> ResponseAsServer {
        unimplemented!()
    }

    async fn map_uri_to_target() -> HashMap<&'static str, &'static str> {
        // TODO: get this map from config
        let mut uri_to_target_map: HashMap<&str, &str> = HashMap::new();
        uri_to_target_map.insert("/home", "http://127.0.0.1:8000");
        uri_to_target_map.insert("/about", "http://127.0.0.1:8000");

        uri_to_target_map
    }
}
