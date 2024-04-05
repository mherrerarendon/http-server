use std::io::Write;

pub trait HttpSerialize {
    fn http_serialize<W: Write>(&self, w: &mut W) -> anyhow::Result<()>;
}

pub trait HttpDeserialize: Sized {
    fn http_deserialize(data: &str) -> anyhow::Result<Self>;
}

#[cfg(test)]
pub mod test_utils {
    macro_rules! serialize_to_str {
        ($serializable:ident) => {{
            let mut s = Vec::new();
            $serializable.http_serialize(&mut s).unwrap();
            String::from_utf8(s).unwrap()
        }};
    }

    pub(crate) use serialize_to_str;
}
