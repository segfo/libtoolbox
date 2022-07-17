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
    #[test]
    fn serialize_to_file_存在しないファイルを作る場合() {
        let serialize_file = "./test_data/serialize.toml";
        assert_eq!(std::path::Path::new(serialize_file).exists(), false);
        let mut conf = Config::new();
        TomlConfigSerializer::to_file(conf.clone(), serialize_file);
        let load_conf: Config = TomlConfigDeserializer::from_file(serialize_file).unwrap();
        assert_eq!(conf, load_conf);
        conf.int32 = 20;
        conf.string = "manipulated.".to_owned();
        TomlConfigSerializer::to_file(conf.clone(), serialize_file);
        let load_conf: Config = TomlConfigDeserializer::from_file(serialize_file).unwrap();
        assert_eq!(conf, load_conf);
        std::fs::remove_file(serialize_file);
    }
    #[test]
    fn serialize_to_file_存在しないディレクトリの下にファイルを作成する場合() {
        let serialize_file = "./test_data/non_exists/serialize.toml";
        let mut conf = Config::new();
        TomlConfigSerializer::to_file(conf.clone(), serialize_file);
        let load_conf: Config = TomlConfigDeserializer::from_file(serialize_file).unwrap();
        assert_eq!(conf, load_conf);
        conf.int32 = 20;
        conf.string = "manipulated.".to_owned();
        TomlConfigSerializer::to_file(conf.clone(), serialize_file);
        let load_conf: Config = TomlConfigDeserializer::from_file(serialize_file).unwrap();
        assert_eq!(conf, load_conf);
        std::fs::remove_file(serialize_file);
        std::fs::remove_dir("./test_data/non_exists/");
        std::fs::remove_dir("./test_data");
    }
    #[test]
    fn deserialize_to_file_存在するディレクトリ_非ファイル_を指定する() {
        let serialize_file = "./test_data/exists_directory";
        assert!(std::path::Path::new(serialize_file).is_dir());
        let conf = Config::new();
        let result = TomlConfigSerializer::to_file(conf.clone(), serialize_file);
        assert!(result.is_err())
    }
}

mod deserializer {
    use crate::tests::toml_parser::Config;
    use crate::toml_parser::{TomlConfigDeserializer, TomlConfigSerializer};
    use std::io::Cursor;
    #[test]
    fn from_file_存在するディレクトリ_非ファイル_を指定する() {
        let serialize_file = "./test_data/exists_directory";
        assert_eq!(std::path::Path::new(serialize_file).is_dir(), true);
        let result: Result<Config, Box<dyn std::error::Error>> =
            TomlConfigDeserializer::from_file(serialize_file);
        assert!(result.is_err())
    }
    #[test]
    fn from_file_存在しないファイルを指定する() {
        let serialize_file = "./test_data/not_exists.toml";
        assert_eq!(std::path::Path::new(serialize_file).exists(), false);
        let result: Result<Config, Box<dyn std::error::Error>> =
            TomlConfigDeserializer::from_file(serialize_file);
        assert!(result.is_err())
    }
}
