use serde::Deserialize;
use serde::Serialize;
use std::fs::File;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Agency{
    pub abbreviation: String,
    pub name: String,
    pub url: String,
    pub api_url: String
}

pub fn get(agency_name: String) -> Option<Agency> {
    let file = File::open("agencies.json").expect("file should open read only");
    let agencies: Vec<Agency> = serde_json::from_reader(file).expect("file should be proper JSON");
    return agencies.into_iter().find(|agency| agency.abbreviation == agency_name);
}

pub fn get_all() -> Vec<Agency> {
    let file = File::open("agencies.json").expect("file should open read only");
    return serde_json::from_reader(file).expect("file should be proper JSON")
}