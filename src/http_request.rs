use anyhow::Result;

use crate::{http_header::HttpHeader, http_method::HttpMethod, http_serde::HttpDeserialize};

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HttpHeader,
    pub body: String,
}

impl HttpRequest {
    fn parse_start_line(start_line: &str) -> Result<(HttpMethod, &str), anyhow::Error> {
        let (verb, rest) = match start_line.split_once(" ") {
            Some(("GET", rest)) => (HttpMethod::GET, rest),
            Some(("POST", rest)) => (HttpMethod::POST, rest),
            Some((&_, _)) => todo!(),
            None => todo!(),
        };

        let (path, _) = rest
            .split_once(" ")
            .ok_or(anyhow::anyhow!("Expected space separator"))?;

        Ok((verb, path))
    }
}

impl HttpDeserialize for HttpRequest {
    fn http_deserialize(request: &str) -> anyhow::Result<Self> {
        let (start_line, rest) = request
            .split_once("\r\n")
            .ok_or(anyhow::anyhow!("Expected line separator"))?;
        let (method, path) = Self::parse_start_line(start_line)?;
        let header_end = rest
            .find("\r\n\r\n")
            .ok_or(anyhow::anyhow!("Expected to find end of header section"))?;
        let header_str = &rest[..header_end];
        let body = &rest[(header_end + 2)..];

        let headers = HttpHeader::http_deserialize(&header_str)?;

        Ok(Self {
            method,
            path: path.to_string(),
            headers,
            body: body.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_deserializes() -> anyhow::Result<()> {
        let request_data =
            "GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\n\r\n";
        let r = HttpRequest::http_deserialize(request_data)?;
        assert_eq!(r.method, HttpMethod::GET);
        assert_eq!(r.path, "/echo/abc");
        assert_eq!(r.headers._count(), 2);
        assert_eq!(r.headers.get("Host").unwrap(), "localhost:4221");
        assert_eq!(r.headers.get("User-Agent").unwrap(), "curl/7.64.1");
        Ok(())
    }
}
