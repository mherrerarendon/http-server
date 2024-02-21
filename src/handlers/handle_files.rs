use std::path::Path;

use crate::http::{
    http_method::HttpMethod, http_request::HttpRequest, http_response::HttpResponse,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub async fn handle_files(request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    let file_path = &request.path[("/files".len())..];
    let abs_file_path = format!("/{}", file_path);
    if request.method == HttpMethod::GET {
        get_response_for_file_get(&abs_file_path).await
    } else if request.method == HttpMethod::POST {
        get_response_for_file_post(request, &abs_file_path).await
    } else {
        Ok(HttpResponse::new_with_status(404))
    }
}

async fn get_response_for_file_get(abs_file_path: &str) -> anyhow::Result<HttpResponse> {
    if Path::new(abs_file_path).exists() {
        let mut file = File::open(abs_file_path).await?;
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

        Ok(response)
    } else {
        Ok(HttpResponse::new_with_status(404))
    }
}

async fn get_response_for_file_post(
    request: &HttpRequest,
    abs_file_path: &str,
) -> anyhow::Result<HttpResponse> {
    let mut file = File::create(abs_file_path).await?;
    file.write_all(request.body.as_bytes()).await?;
    Ok(HttpResponse::new_with_status(201))
}
