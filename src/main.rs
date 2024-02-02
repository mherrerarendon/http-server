use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

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

fn handle_connection(stream: &mut TcpStream) -> anyhow::Result<()> {
    let mut request_bytes = [0u8; 128];
    stream.read(&mut request_bytes)?;
    let request = std::str::from_utf8(&request_bytes)?;
    let (start_line, _) = request
        .split_once("\r\n\r\n")
        .ok_or(anyhow::anyhow!("Expected line separator"))?;
    let (_, path) = parse_start_line(start_line)?;
    let response_code = if path == "/" { "200" } else { "404" };

    let mut response: Vec<String> = Vec::new();
    response.push(format!("HTTP/1.1 {} OK", response_code));
    response.push("Content-Type: text/plain".to_string());
    let (_, random_str_in_path) = path
        .rsplit_once("/")
        .ok_or(anyhow::anyhow!("Expected to find / delimiter"))?;
    response.push(format!("Content-Length: {}", random_str_in_path.len()));
    response.push("".to_string());
    response.push(random_str_in_path.to_string());

    let response_str = response.join("\r\n\r\n");
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
