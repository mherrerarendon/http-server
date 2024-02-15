pub mod handle_echo;
pub mod handle_files;
pub mod handle_not_found;
pub mod handle_root;
pub mod handle_user_agent;

use crate::{
    http::{http_request::HttpRequest, http_serde::HttpDeserialize},
    tcp::read_from_stream_until_null,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn handle_connection(mut stream: TcpStream, dir: String) -> anyhow::Result<()> {
    loop {
        if let Ok(request_bytes) = read_from_stream_until_null(&mut stream).await {
            if request_bytes.len() == 0 {
                println!("Client closed connection");
                break;
            }
            let request = std::str::from_utf8(&request_bytes)?;
            println!("{}", request);
            let request = HttpRequest::http_deserialize(request)?;
            if request.path == "/" {
                handle_root::handle_root(&mut stream).await?;
            } else if request.path.starts_with("/echo") {
                handle_echo::handle_echo(&mut stream, &request).await?;
            } else if request.path.starts_with("/user-agent") {
                handle_user_agent::handle_user_agent(&mut stream, &request).await?;
            } else if request.path.starts_with("/files") {
                handle_files::handle_files(&mut stream, &request, &dir).await?;
            } else {
                handle_not_found::handle_not_found(&mut stream).await?;
            };
        } else {
            println!("Stream timed out ");
            stream.shutdown().await?;
            break;
        }
    }
    Ok(())
}
