use async_std::{io, prelude::*, task};
use log::error;
use std::error::Error;
use surf::{Request as ClientRequest, Response as ClientResponse};
use tide::{Request, Response};
use url::Url;

fn main() {
    task::block_on(async move {
        let addr = format!("127.0.0.1:12666"); //proxy server should listen on here
        let mut server = tide::new();
        server.at("/").all(proxy);
        if let Err(e) = server.listen(addr).await {
            error!("{}", e);
        }
    });
}

async fn proxy(request: Request<()>) -> Response {
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
    let target_server_url = Url::parse("http://127.0.0.1:8000")?;

    let mut target_server_request = ClientRequest::new(request.method().clone(), target_server_url);
    target_server_request = target_server_request.body_bytes(body);

    Ok(target_server_request.await.map_err(|e| format!("{}", e))?)
}
