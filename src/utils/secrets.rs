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
    let secrets_path: String  = std::env::var("SECRETS").expect("No SECRETS environment variable declared");
    let file = File::open(secrets_path).expect("Secrets file, as declared by the SECRETS environment variable, was not found");
    return serde_json::from_reader(file).expect("Secrets file should be proper JSON");
}
