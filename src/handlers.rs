pub mod handle_echo;
pub mod handle_files;
pub mod handle_not_found;
pub mod handle_root;
pub mod handle_user_agent;
pub mod handler;

use crate::{
    http::{http_request::HttpRequest, http_serde::HttpDeserialize},
    tcp::read_from_stream_until_null,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};
