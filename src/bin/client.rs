use http_server_starter_rust::http::{http_request::HttpRequest, http_serde::HttpSerialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4221").await?;
    let mut bytes = Vec::new();
    HttpRequest::default().http_serialize(&mut bytes)?;

    // Write some data.
    stream.write_all(&bytes).await?;

    let mut response_buf = [0u8; 128];
    let response = stream.read(&mut response_buf).await?;
    println!("got {} bytes back", response);
    println!("{}", String::from_utf8(response_buf.to_vec())?);

    Ok(())
}
