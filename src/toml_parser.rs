use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Borrow;
use std::marker::PhantomData;

pub struct TomlConfigDeserializer<T> {
    _phantom: PhantomData<T>,
}

use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, BufWriter};

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

pub struct TomlConfigSerializer<T> {
    _phantom: PhantomData<T>,
}
impl<T> TomlConfigSerializer<T>
where
    T: Serialize,
{
    pub fn to_file(data: T, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let f = OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(filepath)?;
        TomlConfigSerializer::to_writer(&data, f);
        Ok(())
    }
    pub fn to_writer<W>(data: &T, writer: W) -> Result<(), Box<dyn std::error::Error>>
    where
        W: Write,
    {
        let mut bw = BufWriter::new(writer);
        bw.write(toml::to_string(data)?.as_bytes());
        Ok(())
    }
}
