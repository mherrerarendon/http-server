use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::http::{
    http_request::HttpRequest, http_response::HttpResponse, http_serde::HttpSerialize,
};

pub async fn handle_user_agent(
    stream: &mut TcpStream,
    request: &HttpRequest,
) -> anyhow::Result<()> {
    let user_agent = request
        .headers
        .get("User-Agent")
        .ok_or(anyhow::anyhow!("Request does not have User-Agent header"))?;
    let mut response = HttpResponse::new_with_status(200);
    response.headers.add("Content-Type", "text/plain");
    response
        .headers
        .add("Content-Length", &user_agent.len().to_string());
    response.body = user_agent.clone();

    let response_str = response.http_serialize();
    println!("response_str: {}", response_str);

    stream.write_all(response_str.as_bytes()).await?;
    Ok(())
}
