use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::http::{http_response::HttpResponse, http_serde::HttpSerialize};

use super::handler::HttpHandler;

pub struct RootHandler;

impl HttpHandler for RootHandler {
    fn should_handle(&self, r: &crate::http::http_request::HttpRequest) -> bool {
        r.path == "/"
    }

    async fn get_response(
        &self,
        _: &crate::http::http_request::HttpRequest,
    ) -> anyhow::Result<HttpResponse> {
        Ok(HttpResponse::new_with_status(201))
    }
}

pub async fn handle_root(stream: &mut TcpStream) -> anyhow::Result<()> {
    let response_str = HttpResponse::new_with_status(201).http_serialize();
    println!("{}\n///////////////", response_str);

    stream.write_all(response_str.as_bytes()).await?;
    Ok(())
}
