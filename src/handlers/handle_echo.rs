use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::http::{
    http_request::HttpRequest, http_response::HttpResponse, http_serde::HttpSerialize,
};

use super::handler::HttpHandler;

pub struct EchoHandler;

impl HttpHandler for EchoHandler {
    fn should_handle(&self, r: &HttpRequest) -> bool {
        r.path.starts_with("/echo")
    }

    async fn get_response(&self, r: &HttpRequest) -> anyhow::Result<HttpResponse> {
        let (_, response_text) = r.path[1..]
            .split_once("/")
            .ok_or(anyhow::anyhow!("Expected to find delimiter"))?;
        let mut response = HttpResponse::new_with_status(200);
        response.headers.add("Content-Type", "text/plain");
        response
            .headers
            .add("Content-Length", &response_text.len().to_string());
        response.body = response_text.to_string();
        Ok(response)
    }
}

pub async fn handle_echo_again(r: &HttpRequest) -> anyhow::Result<HttpResponse> {
    let (_, response_text) = r.path[1..]
        .split_once("/")
        .ok_or(anyhow::anyhow!("Expected to find delimiter"))?;
    let mut response = HttpResponse::new_with_status(200);
    response.headers.add("Content-Type", "text/plain");
    response
        .headers
        .add("Content-Length", &response_text.len().to_string());
    response.body = response_text.to_string();
    Ok(response)
}

pub async fn handle_echo(stream: &mut TcpStream, request: &HttpRequest) -> anyhow::Result<()> {
    let (_, response_text) = request.path[1..]
        .split_once("/")
        .ok_or(anyhow::anyhow!("Expected to find delimiter"))?;
    let mut response = HttpResponse::new_with_status(200);
    response.headers.add("Content-Type", "text/plain");
    response
        .headers
        .add("Content-Length", &response_text.len().to_string());
    response.body = response_text.to_string();

    let response_str = response.http_serialize();
    println!("{}\n//////////////////", response_str);

    stream.write_all(response_str.as_bytes()).await?;
    Ok(())
}
