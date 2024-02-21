use futures::FutureExt;
use http_server_starter_rust::{
    handlers::{
        handle_echo::handle_echo, handle_files::handle_files, handle_user_agent::handle_user_agent,
    },
    http::http_response::HttpResponse,
    server::Server,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server = Server::new();

    server.add_handler(r"/", |_| {
        async move { Ok(HttpResponse::new_with_status(201)) }.boxed()
    })?;
    server.add_handler(r"/echo", |request| handle_echo(request).boxed())?;
    server.add_handler(r"/files", |request| handle_files(request).boxed())?;
    server.add_handler(r"/user-agent", |request| handle_user_agent(request).boxed())?;

    server.accept_requests().await
}
