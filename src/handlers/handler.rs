use std::future::IntoFuture;

use anyhow::Result;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

use crate::http::{http_request::HttpRequest, http_response::HttpResponse};

use regex::Regex;

pub trait HttpHandler {
    fn should_handle(&self, r: &HttpRequest) -> bool;
    async fn get_response(&self, r: &HttpRequest) -> anyhow::Result<HttpResponse>;
    async fn handle<W>(&self, s: &mut W, r: &HttpRequest) -> anyhow::Result<()>
    where
        W: AsyncWriteExt,
    {
        Ok(())
    }
}
