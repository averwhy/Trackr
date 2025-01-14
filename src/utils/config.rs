// get json data from config.json
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub prefix: String,
}

pub fn get() -> Config {
    let file = File::open("config.json").expect("file should open read only");
    return serde_json::from_reader(file).expect("file should be proper JSON");
}
