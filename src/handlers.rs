mod handle_echo;
mod handle_files;
mod handle_not_found;
mod handle_root;
mod handle_user_agent;

use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{http_request::HttpRequest, http_serde::HttpDeserialize};

pub async fn handle_connection(mut stream: TcpStream, dir: String) -> anyhow::Result<()> {
    let mut request_bytes = [0u8; 1000];
    stream.read(&mut request_bytes).await?;

    let request = std::str::from_utf8(&request_bytes)?;
    let request = HttpRequest::http_deserialize(request)?;
    if request.path == "/" {
        handle_root::handle_root(stream).await?;
    } else if request.path.starts_with("/echo") {
        handle_echo::handle_echo(stream, &request).await?;
    } else if request.path.starts_with("/user-agent") {
        handle_user_agent::handle_user_agent(stream, &request).await?;
    } else if request.path.starts_with("/files") {
        handle_files::handle_files(stream, &request, &dir).await?;
    } else {
        handle_not_found::handle_not_found(stream).await?;
    };

    Ok(())
}
