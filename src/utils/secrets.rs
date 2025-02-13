// get json data from config.json
use crate::Error;
use serde::Deserialize;
use serde::Serialize;
use dotenv::dotenv;
use std::fs::File;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Secrets {
    pub token: String,
    pub database_url: String,
    pub alt_database_url: String,
    pub mbta_api_key: String
}
pub fn get() -> Secrets {
    let get_secrets_path = || -> Result<String, Error> {
        if let Ok(secrets_path) = std::env::var("SECRETS") {
            return Ok(secrets_path);
        }
        dotenv().ok(); // will load the dotenv if the secrets variable is not found normally
        let secrets_path = std::env::var("SECRETS")?;
        Ok(secrets_path)
    };

    let file = File::open(get_secrets_path().unwrap()).expect("Secrets file, as declared by the SECRETS environment variable, was not found");
    return serde_json::from_reader(file).expect("Secrets file should be proper JSON");
}
