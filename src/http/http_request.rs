use std::io::{BufRead, Write};

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
    fn http_serialize<W: Write>(&self, w: &mut W) -> anyhow::Result<()> {
        write!(w, "{} {} HTTP/1.1\r\n", self.method, self.path)?;
        self.headers.http_serialize(w)?;
        write!(w, "\r\n{}", self.body)?;
        Ok(())
    }
}

impl HttpDeserialize for HttpRequest {
    fn http_deserialize<R: BufRead>(r: &mut R) -> anyhow::Result<Self> {
        let mut start_line_buf = String::new();
        r.read_line(&mut start_line_buf)?;
        let (method, path) = Self::parse_start_line(&start_line_buf)?;
        let headers = HttpHeader::http_deserialize(r)?;

        let mut body_buf = String::new();
        r.read_to_string(&mut body_buf)?;

        Ok(Self {
            method,
            path: path.to_string(),
            headers,
            body: body_buf,
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

    use crate::http::http_serde::test_utils::{deserialize_from_str, serialize_to_str};

    use super::*;

    #[test]
    fn it_deserializes_request_with_no_headers() -> anyhow::Result<()> {
        let r = deserialize_from_str!("GET / HTTP/1.1\r\n\r\n" => HttpRequest);
        assert_eq!(r.method, HttpMethod::GET);
        assert_eq!(r.path, "/");
        assert_eq!(r.headers._count(), 0);
        assert_eq!(r.body, "");
        Ok(())
    }

    #[test]
    fn it_deserializes_request_with_headers_and_body() -> anyhow::Result<()> {
        let r = deserialize_from_str!("GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\n\r\nthis is body text\0" => HttpRequest);
        assert_eq!(r.method, HttpMethod::GET);
        assert_eq!(r.path, "/echo/abc");
        assert_eq!(r.headers._count(), 2);
        assert_eq!(r.headers.get("Host").unwrap(), "localhost:4221");
        assert_eq!(r.headers.get("User-Agent").unwrap(), "curl/7.64.1");
        assert_eq!(r.body, "this is body text\0");
        Ok(())
    }

    #[test]
    fn it_deserializes_request_with_headers_and_no_body() -> anyhow::Result<()> {
        let r = deserialize_from_str!("GET /somepath HTTP/1.1\r\nHost: localhost:4221\r\n\r\n" => HttpRequest);
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
        assert_eq!(serialize_to_str!(r), "GET / HTTP/1.1\r\n\r\n");
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
            serialize_to_str!(r),
            "GET /somepath HTTP/1.1\r\nHost: localhost:4221\r\n\r\n"
        );
        Ok(())
    }
}
