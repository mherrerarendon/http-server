use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::http::{http_response::HttpResponse, http_serde::HttpSerialize};

pub async fn handle_not_found(stream: &mut TcpStream) -> anyhow::Result<()> {
    let response_str = HttpResponse::new_with_status(404).http_serialize();
    println!("response_str: {}", response_str);

    stream.write_all(response_str.as_bytes()).await?;
    Ok(())
}
