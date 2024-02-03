use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{http_request::HttpRequest, http_response::HttpResponse, http_serde::HttpDeserialize};

async fn handle_root(stream: &mut TcpStream) -> anyhow::Result<()> {
    let response_str = HttpResponse::new_with_status(200).serialize();
    println!("response_str: {}", response_str);

    stream.write_all(response_str.as_bytes()).await?;
    Ok(())
}

async fn handle_echo(stream: &mut TcpStream, request: &HttpRequest) -> anyhow::Result<()> {
    let (_, response_text) = request.path[1..]
        .split_once("/")
        .ok_or(anyhow::anyhow!("Expected to find delimiter"))?;
    let mut response = HttpResponse::new_with_status(200);
    response
        .headers
        .push("Content-Type: text/plain".to_string());
    response
        .headers
        .push(format!("Content-Length: {}", response_text.len()));
    response.body = response_text.to_string();

    let response_str = response.serialize();
    println!("response_str: {}", response_str);

    stream.write_all(response_str.as_bytes()).await?;
    Ok(())
}

async fn handle_user_agent(stream: &mut TcpStream, request: &HttpRequest) -> anyhow::Result<()> {
    let user_agent = request
        .headers
        .get("User-Agent")
        .ok_or(anyhow::anyhow!("Request does not have User-Agent header"))?;
    let mut response = HttpResponse::new_with_status(200);
    response
        .headers
        .push("Content-Type: text/plain".to_string());
    response
        .headers
        .push(format!("Content-Length: {}", user_agent.len()));
    response.body = user_agent.clone();

    let response_str = response.serialize();
    println!("response_str: {}", response_str);

    stream.write_all(response_str.as_bytes()).await?;
    Ok(())
}

async fn handle_not_found(stream: &mut TcpStream) -> anyhow::Result<()> {
    let response_str = HttpResponse::new_with_status(404).serialize();
    println!("response_str: {}", response_str);

    stream.write_all(response_str.as_bytes()).await?;
    Ok(())
}

pub async fn handle_connection(stream: &mut TcpStream) -> anyhow::Result<()> {
    let mut request_bytes = [0u8; 128];
    let bytes_read = stream.read(&mut request_bytes).await?;
    println!("read {} bytes", bytes_read);

    // let mut buffer = Vec::new();
    // stream.read_to_end(&mut buffer).await?;

    let request = std::str::from_utf8(&request_bytes)?;
    let request = HttpRequest::http_deserialize(request)?;
    println!("handling for path: {}", request.path);
    if request.path == "/" {
        handle_root(stream).await?;
    } else if request.path.starts_with("/echo") {
        handle_echo(stream, &request).await?;
    } else if request.path.starts_with("/user-agent") {
        handle_user_agent(stream, &request).await?;
    } else {
        handle_not_found(stream).await?;
    };

    Ok(())
}
