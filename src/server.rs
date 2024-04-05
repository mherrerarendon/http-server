use std::{future::IntoFuture, sync::Arc};

use anyhow::Context;
use regex::Regex;
use tokio::{
    io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    time::{timeout, Duration},
};

use crate::http::{
    http_request::HttpRequest,
    http_response::HttpResponse,
    http_serde::{HttpDeserialize, HttpSerialize},
};

struct ServerInner {
    handlers: Vec<(
        regex::Regex,
        fn(&HttpRequest) -> futures::future::BoxFuture<anyhow::Result<HttpResponse>>,
    )>,
    debug: bool,
}

impl ServerInner {
    fn log(&self, msg: &str) {
        if self.debug {
            println!("{}", msg);
        }
    }
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
                    i.log("accepted new connection");
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
        stream: TcpStream,
        server_inner: Arc<ServerInner>,
    ) -> anyhow::Result<()> {
        let (reader, mut writer) = split(stream);
        let mut buf_reader = BufReader::new(reader);
        let mut buf = Vec::new();
        const TIMEOUT_DUR: Duration = Duration::from_millis(500);

        loop {
            buf.clear();
            let mut bytes = Vec::new();
            let i = server_inner.clone();
            let n = timeout(TIMEOUT_DUR, buf_reader.read_until(b'\0', &mut buf))
                .await
                .context("Connection timed out")?
                .context("Failed to read bytes")?;
            if n == 0 {
                server_inner.log("Client closed connection");
                break;
            }
            let request = std::str::from_utf8(&buf)?;
            server_inner.log(request);
            let request = HttpRequest::http_deserialize(request)?;
            let response = Server::handle_request(&request, i).await?;
            response.http_serialize(&mut bytes)?;

            writer.write_all(&bytes).await?;
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
