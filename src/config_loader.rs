use std::fs::OpenOptions;
use std::io::Write;
use serde::de::DeserializeOwned;
use dirs::home_dir;
use crate::toml_parser::*;
pub struct ConfigLoader{}
impl ConfigLoader {
    // カレントディレクトリに新規作成を試みる
    // もし新規作成できなければ、ホームディレクトリに作成する
    pub fn load_file<Conf>(path: &str) -> Conf where Conf:Default + DeserializeOwned + serde::Serialize {
        let config = TomlConfigDeserializer::<Conf>::from_file(path);
        match config {
            Ok(file) => file,
            Err(_e) => {
                let conf = Conf::default();
                match OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .read(false)
                    .open(path)
                {
                    Ok(mut file) => {
                        let _r = file.write(toml::to_string(&conf).unwrap().as_bytes());
                    }
                    Err(_e) => {
                        let mut pathbuf = std::fs::canonicalize(&home_dir().unwrap()).unwrap();
                        pathbuf.push(path);
                        match OpenOptions::new()
                            .truncate(true)
                            .create(true)
                            .write(true)
                            .read(false)
                            .open(pathbuf)
                        {
                            Ok(mut file) => {
                                let _r = file.write(toml::to_string(&conf).unwrap().as_bytes());
                            }
                            Err(_e) => {}
                        }
                    }
                }
                conf
            }
        }
    }
}
