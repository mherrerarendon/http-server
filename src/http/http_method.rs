use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpMethod::GET => "GET",
                HttpMethod::POST => "POST",
            }
        )
    }
}
