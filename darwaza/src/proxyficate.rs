use surf::http::request::Request;
use surf::{Request as ClientRequest, Response as ClientResponse};
use tide::{Request as ServerRequest, Response as ServerResponse};
use url::Url;

//TODO: Fix this, not sure how to design this yet.

trait Proxyficate {
    fn convert_downstream_request(rq: ServerRequest<()>) -> (); // TODO: fix this type
    fn convert_upstream_response(rs: ClientResponse) -> ServerResponse;
}
