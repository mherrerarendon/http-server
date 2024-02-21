use crate::http::{http_request::HttpRequest, http_response::HttpResponse};

pub async fn handle_echo(r: &HttpRequest) -> anyhow::Result<HttpResponse> {
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
