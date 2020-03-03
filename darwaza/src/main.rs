use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use futures::TryFutureExt;

fn main() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:12666").await?;
        println!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(incoming_stream) = incoming.next().await {
            let incoming_stream = incoming_stream?;

            task::spawn(async {
                println!("TASK SPAWNING");
                let url = "http://127.0.0.1:8000";
                let mut response = surf::get(url);
                let body = response
                    .recv_string()
                    .await
                    .unwrap_or("Something fail yo!".to_string());
                println!("BODY: {:?}", body);
            });
        }
        Ok(())
    })
}
