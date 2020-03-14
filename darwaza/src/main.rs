use async_std::{io, prelude::*, task};
use http::{Response, Uri};
use log::error;
use std::collections::HashMap;
use std::error::Error;
use surf::{Request as ClientRequest, Response as ClientResponse};
use tide::{Request as ServerRequest, Response as ServerResponse};
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

async fn proxy(request: ServerRequest<()>) -> ServerResponse {
    // get the target server from the uri segment
    // then get the target segment from the remaining url segment

    let mut target_server_response = match request_to_target(request).await {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return ServerResponse::new(500);
        }
    };

    let target_server_response_bytes = match target_server_response.body_bytes().await {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            return ServerResponse::new(500);
        }
    };

    let proxy_response = ServerResponse::new(target_server_response.status().as_u16())
        .body(io::Cursor::new(target_server_response_bytes));

    proxy_response
}

async fn request_to_target(
    mut request: ServerRequest<()>,
) -> Result<ClientResponse, Box<dyn Error>> {
    let body = request.body_bytes().await?;
    let method = request.method().clone();

    let target_server_url = target(request)
        .await
        .unwrap_or(Url::parse("http://127.0.0.1:8000/404.html")?);

    let mut target_server_request = ClientRequest::new(method, target_server_url);
    target_server_request = target_server_request.body_bytes(body);

    Ok(target_server_request.await.map_err(|e| format!("{}", e))?)
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
async fn target(request: ServerRequest<()>) -> Result<Url, Box<dyn Error>> {
    let incoming_request_uri = request.uri().clone();
    // get the target from uri segment using the map
    let uri_map = map_uri_to_target().await;
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
    let mut result_target = Url::parse(result_server.as_str())?.join(result_url.as_str())?;

    dbg!("The final Target>> ", result_target.clone());

    Ok(result_target)
}

async fn map_uri_to_target() -> HashMap<&'static str, &'static str> {
    // TODO: get this map from config
    let mut uri_to_target_map: HashMap<&str, &str> = HashMap::new();
    uri_to_target_map.insert("/home", "http://127.0.0.1:8000");
    uri_to_target_map.insert("/about", "http://127.0.0.1:8000");

    uri_to_target_map
}
