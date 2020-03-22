use std::error::Error;

use http_client::isahc::IsahcClient;
use surf::{Request as RequestAsClient, Response as ResponseAsClient};
use tide::Request as RequestAsServer;
use url::Url;

use crate::urlmap::UrlMap;

#[derive(Debug)]
pub struct Proxyficate {
    pub request_from_client: RequestAsServer<()>,
}

impl Proxyficate {
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use async_std::prelude::*;
    use async_std::task;
    use log::error;
    use tide::Response as ResponseAsServer;

    use super::*;

    #[test]
    fn test_map_request_ok() -> Result<(), surf::Exception> {
        task::block_on(async {
            let server = task::spawn(async {
                let mut app = tide::new();
                app.at("/").get(|mut req: tide::Request<()>| async move {
                    assert_eq!(req.body_string().await.unwrap(), "nori".to_string());

                    let req_method = req.method().clone();

                    let mut p = Proxyficate {
                        request_from_client: req,
                    };

                    let proxy_request = match p.map_request().await {
                        Ok(r) => r,
                        Err(e) => {
                            error!("{}", e);
                            return ResponseAsServer::new(500);
                        }
                    };

                    assert_eq!(proxy_request.method(), req_method);

                    tide::Response::new(200).body_string("says hello".to_string())
                });
                app.listen("localhost:8080").await?;
                Result::<(), surf::Exception>::Ok(())
            });

            let client = task::spawn(async {
                task::sleep(Duration::from_millis(100)).await;

                let string = surf::get("localhost:8080")
                    .body_string("nori".to_string())
                    .recv_string()
                    .await?;
                assert_eq!(string, "says hello".to_string());
                Ok(())
            });

            server.race(client).await
        })
    }
}
