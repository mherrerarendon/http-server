mod handle_connection;
mod http_header;
mod http_method;
mod http_request;
mod http_response;
mod http_serde;

use tokio::net::TcpListener;

use crate::handle_connection::handle_connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    if let Err(err) = handle_connection(stream).await {
                        println!("connection had error: {}", err)
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
