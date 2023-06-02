use serde_json::Value;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader};
use std::collections::HashMap;
use serde::de::DeserializeOwned;

// ファイルから読み込む
pub fn load_file<T:DeserializeOwned>(path:&Path)->std::result::Result<T ,Box<dyn std::error::Error>>{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader)?;
    Ok(result)
}
