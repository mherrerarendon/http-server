use std::io::{BufRead, Write};

pub trait HttpSerialize {
    fn http_serialize<W: Write>(&self, w: &mut W) -> anyhow::Result<()>;
}

pub trait HttpDeserialize: Sized {
    fn http_deserialize<R: BufRead>(r: &mut R) -> anyhow::Result<Self>;
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

    macro_rules! deserialize_from_str {
        ($read_content:expr => $deserializable:ty ) => {{
            use std::io::Cursor;
            let mut r = Cursor::new(String::from($read_content));
            <$deserializable>::http_deserialize(&mut r)?
        }};
    }

    pub(crate) use deserialize_from_str;
    pub(crate) use serialize_to_str;
}
