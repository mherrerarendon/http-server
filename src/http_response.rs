pub struct HttpResponse {
    pub status_line: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl HttpResponse {
    pub fn serialize(&self) -> String {
        let mut headers_str = if self.headers.len() > 0 {
            self.headers.join("\r\n")
        } else {
            "".to_string()
        };
        headers_str.push_str("\r\n");

        format!("{}\r\n{}\r\n{}", self.status_line, headers_str, self.body)
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            status_line: "".to_string(),
            headers: vec![],
            body: "".to_string(),
        }
    }
}
