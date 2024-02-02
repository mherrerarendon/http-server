use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(stream: &mut TcpStream) -> Result<(), anyhow::Error> {
    let mut query_bytes = [0u8; 128];
    stream.read(&mut query_bytes)?;
    stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())?;
    Ok(())
}

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
