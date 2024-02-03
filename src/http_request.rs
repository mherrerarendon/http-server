use std::collections::HashMap;

use anyhow::Result;

use crate::http_method::HttpMethod;

pub struct HttpRequest {
    _method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl HttpRequest {
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

        Ok((verb, path))
    }

    fn parse_header(header_str: &str) -> anyhow::Result<(&str, &str)> {
        println!("parsing header: {}", header_str);
        header_str
            .split_once(": ")
            .ok_or(anyhow::anyhow!("Expected to find header delimiter"))
    }

    pub fn parse(request_bytes: &[u8; 128]) -> anyhow::Result<Self> {
        let request = std::str::from_utf8(request_bytes)?;
        let (start_line, rest) = request
            .split_once("\r\n")
            .ok_or(anyhow::anyhow!("Expected line separator"))?;
        let (method, path) = Self::parse_start_line(start_line)?;

        println!("rest: {}", rest);
        let headers: HashMap<String, String> = if rest.trim() != "" {
            rest.split("\r\n")
                .filter_map(|header| {
                    if header.trim() == "" {
                        None
                    } else {
                        Some(
                            Self::parse_header(header)
                                .and_then(|(key, val)| Ok((key.to_string(), val.to_string()))),
                        )
                    }
                })
                .collect::<Result<Vec<(String, String)>>>()?
                .into_iter()
                .collect()
        } else {
            HashMap::default()
        };

        Ok(Self {
            _method: method,
            path: path.to_string(),
            headers,
        })
    }
}
