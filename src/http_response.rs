pub struct HttpResponse {
    pub status: usize,
    pub headers: Vec<String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new_with_status(status: usize) -> Self {
        Self {
            status,
            ..Default::default()
        }
    }
    pub fn serialize(&self) -> String {
        let status_line = format!("HTTP/1.1 {} OK", self.status);
        let mut headers_str = if self.headers.len() > 0 {
            self.headers.join("\r\n")
        } else {
            "".to_string()
        };
        headers_str.push_str("\r\n");

        format!("{}\r\n{}\r\n{}", status_line, headers_str, self.body)
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            status: 0,
            headers: vec![],
            body: "".to_string(),
        }
    }
}
