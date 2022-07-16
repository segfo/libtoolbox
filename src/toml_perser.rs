use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub struct TomlConfigDeserializer<T> {
    _phantom: PhantomData<T>,
}

use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader};

impl<T> TomlConfigDeserializer<T>
where
    T: DeserializeOwned,
{
    pub fn from_file(filepath: &str) -> Result<T, Box<dyn std::error::Error>> {
        let f = OpenOptions::new()
            .read(true)
            .write(false)
            .create_new(false)
            .open(filepath)?;
        TomlConfigDeserializer::from_reader(f)
    }

    pub fn from_reader<R>(rdr: R) -> Result<T, Box<dyn std::error::Error>>
    where
        R: Read,
    {
        let mut br = BufReader::new(rdr);
        let mut buf = String::new();
        let _ = br.read_to_string(&mut buf);
        let t = toml::from_str(&buf)?;
        Ok(t)
    }
}
