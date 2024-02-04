use std::collections::HashMap;

use http_server_starter_rust::http::{http_request::HttpRequest, http_serde::HttpSerialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{sleep, Duration},
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

    response_buf.iter_mut().for_each(|b| *b = 0);

    sleep(Duration::from_millis(1000)).await;

    let request = HttpRequest {
        path: "/echo/marcoherrerarendon".to_string(),
        headers: HashMap::from([("User-Agent", "marcoclient")]).into(),
        ..Default::default()
    }
    .http_serialize();

    println!("request: \n{}", request);

    // Write some data.
    stream.write_all(request.as_bytes()).await?;

    let bytes_read = stream.read(&mut response_buf).await?;
    if bytes_read == 0 {
        println!("Connection was closed");
        drop(stream);
        println!("Creating new connection and retrying");
        let mut stream = TcpStream::connect("127.0.0.1:4221").await?;
        stream.write_all(request.as_bytes()).await?;
        response_buf.iter_mut().for_each(|b| *b = 0);
        let bytes_read = stream.read(&mut response_buf).await?;
        println!("got {} bytes back", bytes_read);
        println!("{}", String::from_utf8(response_buf.to_vec())?);
    } else {
        println!("got {} bytes back", bytes_read);
        println!("{}", String::from_utf8(response_buf.to_vec())?);
    }

    Ok(())
}
