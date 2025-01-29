// get json data from config.json
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Secrets {
    pub token: String,
    pub database_url: String,
    pub mbta_api_key: String,
}
pub fn get() -> Secrets {
    let file = File::open("secrets.json").expect("file should open read only"); // TODO: convert to env because docker
    return serde_json::from_reader(file).expect("file should be proper JSON");
}
