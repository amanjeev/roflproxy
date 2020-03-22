use std::collections::HashMap;
use std::error::Error;

use http_client::isahc::IsahcClient;
use surf::{Request as RequestAsClient, Response as ResponseAsClient};
use tide::{Request as RequestAsServer, Response as ResponseAsServer};
use url::Url;

use crate::urlmap::UrlMap;

pub struct Proxificate;

impl Proxificate {
    /// Receives request from client, returns corresponding request object
    /// to the downstream server
    pub async fn map_request(
        mut request_from_client: RequestAsServer<()>,
    ) -> RequestAsClient<IsahcClient> {
        let method = request_from_client.method().clone();

        //TODO: Get the target server URL
        let target_server_url = Url::get_target_server_url(request_from_client)
            .unwrap_or(Url::parse("http://127.0.0.1:8000/404.html").unwrap());

        let request_for_downstream = RequestAsClient::new(method, target_server_url);

        request_for_downstream
    }

    pub async fn convert_to_response(
        request_for_downstream: RequestAsClient<IsahcClient>,
    ) -> Result<ResponseAsClient, Box<dyn Error>> {
        Ok(request_for_downstream.await.map_err(|e| format!("{}", e))?)
    }
}
