use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{BufRead, Write},
};

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

    fn add_from_line(&mut self, header_line: &str) -> anyhow::Result<()> {
        let (key, val) = HttpHeader::parse_header(header_line)?;
        self.headers.insert(key.to_string(), val.to_string());
        Ok(())
    }
}

impl HttpSerialize for HttpHeader {
    fn http_serialize<W: Write>(&self, w: &mut W) -> anyhow::Result<()> {
        write!(
            w,
            "{}",
            self.headers
                .iter()
                .map(|(key, val)| format!("{key}: {val}\r\n"))
                .join("")
        )?;
        Ok(())
    }
}

impl HttpDeserialize for HttpHeader {
    fn http_deserialize<R: BufRead>(r: &mut R) -> Result<Self> {
        let mut headers = HttpHeader::default();
        let mut buf = String::new();
        while let Ok(_) = r.read_line(&mut buf) {
            let s = buf.trim();
            if s.is_empty() {
                break;
            }
            headers.add_from_line(s)?;
            buf.clear();
        }
        Ok(headers)
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
    use crate::http::http_serde::test_utils::deserialize_from_str;

    use super::*;

    #[test]
    fn it_deserializes_empty_header_section() -> anyhow::Result<()> {
        let h = deserialize_from_str!("" => HttpHeader);
        assert_eq!(h._count(), 0);
        Ok(())
    }

    #[test]
    fn it_deserializes_header_section() -> anyhow::Result<()> {
        let h = deserialize_from_str!("Host: localhost:4221\r\nUser-Agent: curl/7.64.1\r\n" => HttpHeader);
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

        let mut s = Vec::new();

        h.http_serialize(&mut s)?;
        let s = String::from_utf8(s)?;

        // Since it starts out as a hashmap, the order is not determined
        if s.starts_with("User-Agent") {
            assert_eq!(s, "User-Agent: curl/7.64.1\r\nHost: localhost:4221\r\n");
        } else {
            assert_eq!(s, "Host: localhost:4221\r\nUser-Agent: curl/7.64.1\r\n");
        }
        Ok(())
    }
}
