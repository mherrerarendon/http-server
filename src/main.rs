mod handle_connection;
mod http_header;
mod http_method;
mod http_request;
mod http_response;
mod http_serde;

use std::net::TcpListener;

use crate::handle_connection::handle_connection;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for mut stream in listener.incoming() {
        match stream {
            Ok(ref mut stream) => {
                println!("accepted new connection");
                if let Err(err) = handle_connection(stream) {
                    println!("connection had error: {}", err)
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
