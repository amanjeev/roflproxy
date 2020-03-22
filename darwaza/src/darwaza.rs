use std::collections::HashMap;
use std::error::Error;

use async_std::{io, task};
use log::error;
use surf::{Request as RequestAsClient, Response as ResponseAsClient};
use tide::{Request as RequestAsServer, Response as ResponseAsServer};
use url::Url;

use crate::proxyficate::Proxificate;

pub fn demain() {
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
    })
}

async fn proxy(request: RequestAsServer<()>) -> ResponseAsServer {
    // get the target server from the uri segment
    // then get the target segment from the remaining url segment

    let mut p = Proxificate {
        request_from_client: request,
    };

    let mut target_server_response = match p.convert_to_response().await {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return ResponseAsServer::new(500);
        }
    };

    let target_server_response_bytes = match target_server_response.body_bytes().await {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            return ResponseAsServer::new(500);
        }
    };

    let proxy_response = ResponseAsServer::new(target_server_response.status().as_u16())
        .body(io::Cursor::new(target_server_response_bytes));

    proxy_response
}
