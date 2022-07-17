use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct Config {
    int32: i32,
    vec_int32: Vec<i32>,
    string: String,
}
impl Config {
    fn new() -> Self {
        Config {
            int32: 10,
            vec_int32: vec![10, 20, 30, 40, 50],
            string: "test_string".to_owned(),
        }
    }
    fn get_serialize_data() -> String {
        "int32 = 10\nvec_int32 = [10, 20, 30, 40, 50]\nstring = \"test_string\"\n".to_owned()
    }
}

#[cfg(test)]
mod serializer {
    use std::io::Cursor;

    use crate::tests::toml_parser::Config;
    use crate::toml_parser::{TomlConfigDeserializer, TomlConfigSerializer};
    #[test]
    fn serialize() {
        let conf = Config::new();
        let mut v = Vec::new();
        TomlConfigSerializer::to_writer(&conf, &mut v);
        assert_eq!(String::from_utf8(v).unwrap(), Config::get_serialize_data());
    }
    #[test]
    fn deserialize() {
        let conf = Config::new();
        let v = Config::get_serialize_data();
        let rd = Cursor::new(v);
        let config: Result<Config, _> = TomlConfigDeserializer::from_reader(rd);
        assert_eq!(config.is_ok(), true);
        assert_eq!(conf, config.unwrap());
    }
}
