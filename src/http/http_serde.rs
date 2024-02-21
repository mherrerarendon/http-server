pub trait HttpSerialize {
    fn http_serialize(&self) -> String;
}

pub trait HttpDeserialize: Sized {
    fn http_deserialize(data: &str) -> anyhow::Result<Self>;
}
