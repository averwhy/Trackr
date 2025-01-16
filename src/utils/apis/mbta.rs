use reqwest;
use serde_json::from_str;
use crate::Error;
use crate::utils::apis::structs::mbta_data;

pub struct MBTA{
    http: reqwest::Client,
    pub url: String
}

impl MBTA {
    pub fn new(key: String) -> Self {
        let url = "https://api-v3.mbta.com"; // TODO; grab api url from agencies.json
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert("x-api-key", reqwest::header::HeaderValue::from_str(&key).expect("Invalid API key"));
        let http_client = reqwest::ClientBuilder::new().https_only(true).default_headers(default_headers).build().expect("http client build failed");
        Self { http: http_client, url: url.to_string() }
    }

    pub async fn get_alerts(&self) -> Result<mbta_data::Root, Error> {
        let response = self.http
            .get(format!("{}/alerts", self.url))
            .send()
            .await?
            .text()
            .await?;
        
        dbg!(response[115..135].to_string());

        let alerts: mbta_data::Root = from_str(&response)?;
        Ok(alerts)
    }
}