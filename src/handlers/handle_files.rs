use std::path::Path;

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{http_request::HttpRequest, http_response::HttpResponse, http_serde::HttpSerialize};

use super::handle_not_found::handle_not_found;

pub async fn handle_files(
    mut stream: TcpStream,
    request: &HttpRequest,
    dir: &str,
) -> anyhow::Result<()> {
    let file_path = &request.path[("/files".len())..];
    let abs_file_path = format!("{}{}", dir, file_path);
    if Path::new(&abs_file_path).exists() {
        let mut file = File::open(&abs_file_path).await?;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await?;
        let contents = String::from_utf8(contents)?;

        let mut response = HttpResponse::new_with_status(200);
        response
            .headers
            .add("Content-Type", "application/octet-stream");
        response
            .headers
            .add("Content-Length", &contents.len().to_string());
        response.body = contents;

        let response_str = response.http_serialize();
        println!("response_str: {}", response_str);

        stream.write_all(response_str.as_bytes()).await?;
    } else {
        handle_not_found(stream).await?;
    }

    Ok(())
}
