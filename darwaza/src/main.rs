use async_std::{io, prelude::*, task};
use http::Uri;
use log::error;
use std::collections::HashMap;
use std::error::Error;
use surf::{Request as ClientRequest, Response as ClientResponse};
use tide::{Request, Response};
use url::Url;

fn main() {
    task::block_on(async move {
        // TODO: from config proxy server should listen on here
        let addr = format!("127.0.0.1:12666");
        let mut server = tide::new();

        // let's forward all requests to the `proxy` function
        // and sort out there
        server.at("/").all(proxy);
        server.at("/*").all(proxy);

        if let Err(e) = server.listen(addr).await {
            error!("{}", e);
        }
    });
}

async fn map_uri_to_target() -> HashMap<&'static str, &'static str> {
    // TODO: get this map from config
    let mut uri_to_target_map: HashMap<&str, &str> = HashMap::new();
    uri_to_target_map.insert("/home", "127.0.0.1:8000");
    uri_to_target_map.insert("/about", "127.0.0.1:8000");

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
/// then, the target server is `foo.com:1234`
async fn target_server(request: Request<()>) -> Result<Url, Box<dyn Error>> {
    // get the target from uri segment using the map
    let uri_map = map_uri_to_target().await;
    let mut result_target = "";
    for (uri, server) in uri_map.iter() {
        if request.uri().to_string().contains(uri) {
            result_target = server;
        }
    }
    Ok(Url::parse(result_target)?)
}

/// Returns the target segment after removing the mapping segment
/// from the URL to the target server
/// If the mapping is
///     ```
///     "/home"  => "foo.com:1234"
///     "/staff" => "bar.org:4567"
///     ```
/// and the request is for
///     `/staff/jdoe`
/// then, the target segment is `/jdoe`
async fn target_segment(uri: &Uri) -> Result<Url, Box<dyn Error>> {
    // get the url segment for the target server, minus the segment used
    // to map the target
    let uri_map = map_uri_to_target().await;
    let mut result_url = "";
    for (u, server) in uri_map.iter() {
        if uri.to_string().contains(u) {
            result_url = "";
        }
    }
    let result_url = result_url.replace(uri.to_string().as_str(), result_url);
    let result_url = result_url.as_str();
    Ok(Url::parse(result_url)?)
}

async fn proxy(request: Request<()>) -> Response {
    // get the target server from the uri segment
    // then get the target segment from the remaining url segment

    let mut target_server_response = match request_to_target(request).await {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return Response::new(500);
        }
    };

    let target_server_response_bytes = match target_server_response.body_bytes().await {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            return Response::new(500);
        }
    };

    let proxy_response = Response::new(target_server_response.status().as_u16())
        .body(io::Cursor::new(target_server_response_bytes));

    proxy_response
}

async fn request_to_target(mut request: Request<()>) -> Result<ClientResponse, Box<dyn Error>> {
    let body = request.body_bytes().await?;
    //let uri_seg = request.uri();

    let mut target_server_url = Url::parse("http://127.0.0.1:8000")?;
    target_server_url = target_server_url.join(request.uri().to_string().as_str())?;

    let mut target_server_request = ClientRequest::new(request.method().clone(), target_server_url);
    target_server_request = target_server_request.body_bytes(body);

    Ok(target_server_request.await.map_err(|e| format!("{}", e))?)
}
