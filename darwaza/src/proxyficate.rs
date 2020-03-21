use surf::{Request as RequestAsClient, Response as ResponseAsClient};
use tide::{Request as RequestAsServer, Response as ResponseAsServer};

/// Receives request from client, returns corresponding request object
/// to the downstream server
pub fn map_request(
    request_from_client: RequestAsServer<()>,
) -> RequestAsClient<http_client::isahc::IsahcClient> {
    unimplemented!()
}

/// Receives response from downstream server, returns corresponding
/// response object to the client
pub fn map_response(response_from_downstream: ResponseAsClient) -> ResponseAsServer {
    unimplemented!()
}
