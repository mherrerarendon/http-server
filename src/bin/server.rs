use http_server_starter_rust::handlers::handle_connection;
use std::env;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let directory = if args.len() == 3 { &args[2] } else { "" };

    let listener = TcpListener::bind("127.0.0.1:4221").await?;
    println!("Listening on 127.0.0.1:4221");

    loop {
        let directory = directory.to_string();
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    if let Err(err) = handle_connection(stream, directory).await {
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
