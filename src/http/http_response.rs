use std::io::Write;

use super::{http_header::HttpHeader, http_serde::HttpSerialize};

pub struct HttpResponse {
    pub status: usize,
    pub headers: HttpHeader,
    pub body: String,
}

impl HttpResponse {
    pub fn new_with_status(status: usize) -> Self {
        Self {
            status,
            ..Default::default()
        }
    }
}

impl HttpSerialize for HttpResponse {
    fn http_serialize<W: Write>(&self, w: &mut W) -> anyhow::Result<()> {
        write!(w, "HTTP/1.1 {} OK\r\n", self.status)?;
        self.headers.http_serialize(w)?;
        write!(w, "\r\n{}", self.body)?;
        Ok(())
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            status: 0,
            headers: HttpHeader::default(),
            body: "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::http_serde::test_utils::serialize_to_str;

    #[test]
    fn it_serializes_response_with_no_header() -> anyhow::Result<()> {
        let r = HttpResponse::new_with_status(200);
        assert_eq!(serialize_to_str!(r), "HTTP/1.1 200 OK\r\n\r\n");
        Ok(())
    }

    #[test]
    fn it_serializes_response_with_header() {
        let mut r = HttpResponse::new_with_status(200);
        r.headers.add("Host", "localhost:4221");
        assert_eq!(
            serialize_to_str!(r),
            "HTTP/1.1 200 OK\r\nHost: localhost:4221\r\n\r\n"
        )
    }
}
