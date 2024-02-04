pub mod handle_echo;
pub mod handle_files;
pub mod handle_not_found;
pub mod handle_root;
pub mod handle_user_agent;

use crate::http::{http_request::HttpRequest, http_serde::HttpDeserialize};
use tokio::{io::AsyncReadExt, net::TcpStream, time::timeout};

async fn _read_until_null(stream: &mut TcpStream) -> anyhow::Result<Vec<u8>> {
    const BUFF_SIZE: usize = 5;
    let mut request_buff = [0u8; BUFF_SIZE];
    let mut request_bytes: Vec<u8> = Vec::new();
    loop {
        stream.read(&mut request_buff).await?;
        request_bytes.extend_from_slice(&request_buff);
        if request_buff[BUFF_SIZE - 1] == 0 {
            break;
        }
    }
    Ok(request_bytes)
}

pub async fn handle_connection(mut stream: TcpStream, dir: String) -> anyhow::Result<()> {
    let my_duration = tokio::time::Duration::from_millis(500);
    loop {
        let mut request_bytes = [0u8; 1000];
        let bytes_read = match timeout(my_duration, stream.read(&mut request_bytes)).await {
            Ok(bytes_read) => Some(bytes_read?),
            Err(_) => None,
        };
        match bytes_read {
            None => {
                println!("Stream timed out after {} seconds", my_duration.as_secs());
                break;
            }
            Some(bytes_read) => {
                println!("read {} bytes", bytes_read);
                if bytes_read == 0 {
                    println!("Client closed connection");
                    break;
                }
            }
        };
        let request = std::str::from_utf8(&request_bytes)?;
        println!("request:\n{}", request);
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
    }
    Ok(())
}
