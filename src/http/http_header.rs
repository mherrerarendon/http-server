use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

use super::http_serde::{HttpDeserialize, HttpSerialize};

pub struct HttpHeader {
    headers: HashMap<String, String>,
}

impl HttpHeader {
    fn parse_header(header_str: &str) -> anyhow::Result<(&str, &str)> {
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

    pub fn add(&mut self, key: &str, val: &str) {
        self.headers.insert(key.to_string(), val.to_string());
    }
}

impl HttpSerialize for HttpHeader {
    fn http_serialize(&self) -> String {
        self.headers
            .iter()
            .map(|(key, val)| format!("{key}: {val}\r\n"))
            .join("")
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

impl<S: AsRef<str>> From<HashMap<S, S>> for HttpHeader {
    fn from(h: HashMap<S, S>) -> Self {
        Self {
            headers: h
                .iter()
                .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
                .collect(),
        }
    }
}

impl Default for HttpHeader {
    fn default() -> Self {
        Self {
            headers: HashMap::default(),
        }
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

    #[test]
    fn it_serializes_header_section() -> anyhow::Result<()> {
        let h: HttpHeader = HashMap::from(
            [("Host", "localhost:4221"), ("User-Agent", "curl/7.64.1")]
                .map(|(key, val)| (key.to_string(), val.to_string())),
        )
        .into();

        // sanity check
        assert_eq!(h._count(), 2);

        let s = h.http_serialize();

        // Since it starts out as a hashmap, the order is not determined
        if s.starts_with("User-Agent") {
            assert_eq!(
                h.http_serialize(),
                "User-Agent: curl/7.64.1\r\nHost: localhost:4221\r\n"
            );
        } else {
            assert_eq!(
                h.http_serialize(),
                "Host: localhost:4221\r\nUser-Agent: curl/7.64.1\r\n"
            );
        }
        Ok(())
    }
}
