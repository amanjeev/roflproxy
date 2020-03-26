use crate::config;
use crate::proxyficate::Proxyficate;
use async_std::{io, task};
use log::error;
use tide::{Request as RequestAsServer, Response as ResponseAsServer};

pub fn demain() {
    let config = config::ServerConfig::new();

    task::block_on(async move {
        let addr = config.addr;
        let mut server = tide::new();

        // let's forward all requests to the `proxy` function
        // and sort out there
        server.at("/").all(proxy);
        server.at("/*").all(proxy);

        if let Err(e) = server.listen(addr).await {
            error!("{}", e);
        }
        println!("Listening on: {:?}", addr.clone());
    })
}

async fn proxy(request: RequestAsServer<()>) -> ResponseAsServer {
    // get the target server from the uri segment
    // then get the target segment from the remaining url segment

    let mut p = Proxyficate {
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
