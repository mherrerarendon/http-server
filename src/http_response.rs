use crate::{http_header::HttpHeader, http_serde::HttpSerialize};

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
    fn http_serialize(&self) -> String {
        let status_line = format!("HTTP/1.1 {} OK", self.status);
        let headers_str = self.headers.http_serialize();
        format!("{}\r\n{}\r\n{}", status_line, headers_str, self.body)
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

    #[test]
    fn it_serializes_response_with_no_header() {
        let r = HttpResponse::new_with_status(200);
        assert_eq!(r.http_serialize(), "HTTP/1.1 200 OK\r\n\r\n")
    }

    #[test]
    fn it_serializes_response_with_header() {
        let mut r = HttpResponse::new_with_status(200);
        r.headers.add("Host", "localhost:4221");
        assert_eq!(
            r.http_serialize(),
            "HTTP/1.1 200 OK\r\nHost: localhost:4221\r\n\r\n"
        )
    }
}
