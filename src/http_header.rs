use anyhow::Result;
use std::collections::HashMap;

use crate::http_serde::HttpDeserialize;

pub struct HttpHeader {
    headers: HashMap<String, String>,
}

impl HttpHeader {
    fn parse_header(header_str: &str) -> anyhow::Result<(&str, &str)> {
        println!("parsing header: {}", header_str);
        header_str
            .split_once(": ")
            .ok_or(anyhow::anyhow!("Expected to find header delimiter"))
    }

    pub fn _count(&self) -> usize {
        self.headers.len()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
}

impl HttpDeserialize for HttpHeader {
    fn http_deserialize(data: &str) -> Result<Self> {
        let headers: HashMap<String, String> = if data.trim() != "" {
            data.split("\r\n")
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
        Ok(headers.into())
    }
}

impl From<HashMap<String, String>> for HttpHeader {
    fn from(headers: HashMap<String, String>) -> Self {
        Self { headers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_deserializes_empty_header_section() -> anyhow::Result<()> {
        let h = HttpHeader::http_deserialize(&"")?;
        assert_eq!(h._count(), 0);
        Ok(())
    }

    #[test]
    fn it_deserializes_header_section() -> anyhow::Result<()> {
        let h =
            HttpHeader::http_deserialize(&"Host: localhost:4221\r\nUser-Agent: curl/7.64.1\r\n")?;
        assert_eq!(h._count(), 2);
        Ok(())
    }
}
