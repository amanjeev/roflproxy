use std::collections::HashMap;
use std::error::Error;

use http_client::isahc::IsahcClient;
use surf::{Request as RequestAsClient, Response as ResponseAsClient};
use tide::{Request as RequestAsServer, Response as ResponseAsServer};
use url::Url;

use crate::urlmap::UrlMap;

#[derive(Debug)]
pub struct Proxificate {
    pub request_from_client: RequestAsServer<()>,
}

impl Proxificate {
    /// Receives request from client, returns corresponding request object
    /// to the downstream server
    pub async fn map_request(&mut self) -> Result<RequestAsClient<IsahcClient>, Box<dyn Error>> {
        let method = self.request_from_client.method().clone();

        let target_server_url = Url::get_target_server_url(&self.request_from_client)
            .unwrap_or(Url::parse("http://127.0.0.1:8000/404.html").unwrap());

        let request_for_downstream = RequestAsClient::new(method, target_server_url);

        Ok(request_for_downstream)
    }

    /// Returns the response from the downstream server
    pub async fn convert_to_response(&mut self) -> Result<ResponseAsClient, Box<dyn Error>> {
        let body = self.request_from_client.body_bytes().await.unwrap();
        let mut request_for_downstream = self.map_request().await.unwrap();
        request_for_downstream = request_for_downstream.body_bytes(body);

        Ok(request_for_downstream.await.map_err(|e| format!("{}", e))?)
    }
}
