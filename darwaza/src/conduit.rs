use surf::{Request as ClientRequest, Response as ClientResponse};
use tide::{Request as ServerRequest, Response as ServerResponse};
use url::Url;

/// Proxy as a server
/// Incoming request from a client and outgoing response to that client
struct AsServer {
    //request: ServerRequest,
//headers: ???,
//response: ServerResponse,
}

/// Proxy as a client
/// Making a request to the upstream server and receiving response from that server
struct AsClient {
    //request: ClientRequest
//headers: ???,
//response: ClientResponse,
}

/// Request-Response chain HTTP header structure
struct Headers {}
