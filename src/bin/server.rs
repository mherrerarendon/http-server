use futures::FutureExt;
use http_server_starter_rust::{
    handlers::handle_echo::handle_echo_again, http::http_response::HttpResponse, server::Server,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server = Server::new();

    server.add_handler(r"/", |_| {
        async move { Ok(HttpResponse::new_with_status(201)) }.boxed()
    })?;
    server.add_handler(r"/echo", |request| handle_echo_again(request).boxed())?;

    server.accept_requests().await
}
