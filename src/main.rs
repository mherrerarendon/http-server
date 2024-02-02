mod http_response;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::http_response::HttpResponse;

#[derive(Debug)]
enum HttpMethod {
    GET,
}

fn parse_start_line(start_line: &str) -> Result<(HttpMethod, &str), anyhow::Error> {
    let (verb, rest) = match start_line.split_once(" ") {
        Some(("GET", rest)) => (HttpMethod::GET, rest),
        Some((&_, _)) => todo!(),
        None => todo!(),
    };
    println!("verb: {:?}", verb);

    let (path, _) = rest
        .split_once(" ")
        .ok_or(anyhow::anyhow!("Expected space separator"))?;
    println!("path: {}", path);

    // TODO: parse remainder of start line

    Ok((verb, path))
}

fn resolve_path(path: &str) -> (String, String) {
    let r = if path == "/" {
        ("200", "")
    } else {
        match path[1..].split_once("/") {
            Some(("echo", rest)) => ("200", rest),
            Some((_, _)) => ("404", ""),
            None => ("404", ""),
        }
    };
    (r.0.to_string(), r.1.to_string())
}

fn handle_connection(stream: &mut TcpStream) -> anyhow::Result<()> {
    let mut request_bytes = [0u8; 128];
    stream.read(&mut request_bytes)?;
    let request = std::str::from_utf8(&request_bytes)?;
    let (start_line, _) = request
        .split_once("\r\n")
        .ok_or(anyhow::anyhow!("Expected line separator"))?;
    let (_, path) = parse_start_line(start_line)?;
    let (response_code, response_text) = resolve_path(path);

    let mut response = HttpResponse::default();
    response.status_line = format!("HTTP/1.1 {} OK", response_code);
    response
        .headers
        .push("Content-Type: text/plain".to_string());
    response
        .headers
        .push(format!("Content-Length: {}", response_text.len()));
    response.body = response_text.to_string();

    let response_str = response.serialize();
    println!("response_str: {}", response_str);

    stream.write(response_str.as_bytes())?;
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
