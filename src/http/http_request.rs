use anyhow::Result;

use super::{
    http_header::HttpHeader,
    http_method::HttpMethod,
    http_serde::{HttpDeserialize, HttpSerialize},
};

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

impl HttpSerialize for HttpRequest {
    fn http_serialize(&self) -> String {
        let s = format!("{} {} HTTP/1.1", self.method, self.path);
        let headers_str = self.headers.http_serialize();
        format!("{}\r\n{}\r\n{}", s, headers_str, self.body)
    }
}

impl HttpDeserialize for HttpRequest {
    fn http_deserialize(request: &str) -> anyhow::Result<Self> {
        let (start_line, rest) = request
            .split_once("\r\n")
            .ok_or(anyhow::anyhow!("Expected line separator"))?;
        let (method, path) = Self::parse_start_line(start_line)?;
        let (headers, header_end) = match rest.find("\r\n\r\n") {
            Some(header_end) => {
                let header_str = &rest[..header_end];
                (HttpHeader::http_deserialize(&header_str)?, Some(header_end))
            }
            None => (HttpHeader::default(), None),
        };
        let body = match header_end {
            Some(header_end) => {
                let body = &rest[(header_end + 4)..];
                match body.find("\0") {
                    Some(body_end) => &body[..body_end],
                    None => body,
                }
            }
            None => "",
        };

        Ok(Self {
            method,
            path: path.to_string(),
            headers,
            body: body.to_string(),
        })
    }
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::GET,
            path: "/".to_string(),
            headers: HttpHeader::default(),
            body: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn it_deserializes_request_with_no_headers() -> anyhow::Result<()> {
        let request_data = "GET / HTTP/1.1\r\n\r\n";
        let r = HttpRequest::http_deserialize(request_data)?;
        assert_eq!(r.method, HttpMethod::GET);
        assert_eq!(r.path, "/");
        assert_eq!(r.headers._count(), 0);
        assert_eq!(r.body, "");
        Ok(())
    }

    #[test]
    fn it_deserializes_request_with_headers_and_body() -> anyhow::Result<()> {
        let request_data =
            "GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\n\r\nthis is body text\0";
        let r = HttpRequest::http_deserialize(request_data)?;
        assert_eq!(r.method, HttpMethod::GET);
        assert_eq!(r.path, "/echo/abc");
        assert_eq!(r.headers._count(), 2);
        assert_eq!(r.headers.get("Host").unwrap(), "localhost:4221");
        assert_eq!(r.headers.get("User-Agent").unwrap(), "curl/7.64.1");
        assert_eq!(r.body, "this is body text");
        Ok(())
    }

    #[test]
    fn it_deserializes_request_with_headers_and_no_body() -> anyhow::Result<()> {
        let request_data = "GET /somepath HTTP/1.1\r\nHost: localhost:4221\r\n\r\n";
        let r = HttpRequest::http_deserialize(request_data)?;
        assert_eq!(r.method, HttpMethod::GET);
        assert_eq!(r.path, "/somepath");
        assert_eq!(r.headers._count(), 1);
        assert_eq!(r.headers.get("Host").unwrap(), "localhost:4221");
        assert_eq!(r.body, "");
        Ok(())
    }

    #[test]
    fn it_serializes_default_request() -> anyhow::Result<()> {
        let r = HttpRequest::default();
        assert_eq!(r.http_serialize(), "GET / HTTP/1.1\r\n\r\n");
        Ok(())
    }

    #[test]
    fn it_serializes_request_with_headers() -> anyhow::Result<()> {
        let r = HttpRequest {
            path: "/somepath".to_string(),
            headers: HashMap::from([("Host", "localhost:4221")]).into(),
            ..HttpRequest::default()
        };
        assert_eq!(
            r.http_serialize(),
            "GET /somepath HTTP/1.1\r\nHost: localhost:4221\r\n\r\n"
        );
        Ok(())
    }
}
