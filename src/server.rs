use std::{future::IntoFuture, sync::Arc};

use regex::Regex;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

use crate::{
    http::{
        http_request::HttpRequest,
        http_response::HttpResponse,
        http_serde::{HttpDeserialize, HttpSerialize},
    },
    tcp::read_from_stream_until_null,
};

struct ServerInner {
    handlers: Vec<(
        regex::Regex,
        fn(&HttpRequest) -> futures::future::BoxFuture<anyhow::Result<HttpResponse>>,
    )>,
    debug: bool,
}

impl Default for ServerInner {
    fn default() -> Self {
        Self {
            handlers: vec![],
            debug: false,
        }
    }
}

pub struct Server {
    inner: ServerInner,
}

impl Server {
    pub fn new() -> Self {
        Self {
            inner: ServerInner::default(),
        }
    }

    pub fn add_handler(
        &mut self,
        re: &str,
        handler: fn(&HttpRequest) -> futures::future::BoxFuture<anyhow::Result<HttpResponse>>,
    ) -> anyhow::Result<()> {
        let re = Regex::new(re)?;
        self.inner.handlers.push((re, handler));
        Ok(())
    }

    pub async fn accept_requests(mut self) -> anyhow::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:4221").await?;
        println!("Listening on 127.0.0.1:4221");

        let inner = Arc::new(std::mem::take(&mut self.inner));
        loop {
            let i = inner.clone();
            match listener.accept().await {
                Ok((stream, _)) => {
                    println!("accepted new connection");
                    tokio::spawn(async move {
                        if let Err(err) = Server::handle_connection(stream, i).await {
                            println!("connection had error: {}", err)
                        }
                    });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }

    async fn handle_connection(
        mut stream: TcpStream,
        server_inner: Arc<ServerInner>,
    ) -> anyhow::Result<()> {
        loop {
            let i = server_inner.clone();
            if let Ok(request_bytes) = read_from_stream_until_null(&mut stream).await {
                if request_bytes.len() == 0 {
                    println!("Client closed connection");
                    break;
                }
                let request = std::str::from_utf8(&request_bytes)?;
                println!("{}", request);
                let request = HttpRequest::http_deserialize(request)?;
                let response = Server::handle_request(&request, i).await?;
                let response_str = response.http_serialize();
                println!("response_str: {}", response_str);

                stream.write_all(response_str.as_bytes()).await?;
            } else {
                println!("Stream timed out");
                stream.shutdown().await?;
                break;
            }
        }
        Ok(())
    }

    async fn handle_request(
        request: &HttpRequest,
        server_inner: Arc<ServerInner>,
    ) -> anyhow::Result<HttpResponse> {
        if let Some((_, handler)) = server_inner
            .handlers
            .iter()
            .find(|(re, _)| re.is_match(&request.path))
        {
            handler(request).into_future().await
        } else {
            Ok(HttpResponse::new_with_status(404))
        }
    }
}
