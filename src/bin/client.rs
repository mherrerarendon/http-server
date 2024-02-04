use std::collections::HashMap;

use http_server_starter_rust::http::{http_request::HttpRequest, http_serde::HttpSerialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4221").await?;
    let request = HttpRequest::default().http_serialize();
    println!("request: \n{}", request);

    // Write some data.
    stream.write_all(request.as_bytes()).await?;

    let mut response_buf = [0u8; 128];
    let response = stream.read(&mut response_buf).await?;
    println!("got {} bytes back", response);
    println!("{}", String::from_utf8(response_buf.to_vec())?);

    let request = HttpRequest {
        path: "/echo/marcoherrerarendon".to_string(),
        headers: HashMap::from([("User-Agent", "marcoclient")]).into(),
        ..Default::default()
    }
    .http_serialize();

    println!("request: \n{}", request);

    // Write some data.
    stream.write_all(request.as_bytes()).await?;

    let response = stream.read(&mut response_buf).await?;
    println!("got {} bytes back", response);
    println!("{}", String::from_utf8(response_buf.to_vec())?);

    Ok(())
}
