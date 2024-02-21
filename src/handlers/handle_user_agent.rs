use crate::http::{http_request::HttpRequest, http_response::HttpResponse};

pub async fn handle_user_agent(request: &HttpRequest) -> anyhow::Result<HttpResponse> {
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
    Ok(response)
}
