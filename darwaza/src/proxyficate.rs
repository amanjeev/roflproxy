use std::error::Error;
use surf::http::request::Request;
use surf::{Request as RequestAsClient, Response as ResponseAsClient};
use tide::{Request as RequestAsServer, Response as ResponseAsServer};
use url::Url;

pub trait Proxificate {
    fn derive_request_to_server(mut request_from_client: RequestAsServer<()>) -> () {}
}
