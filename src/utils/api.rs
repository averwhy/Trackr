use crate::utils::database::Client;
use crate::utils::database::EndpointType;
use crate::Error;
use reqwest::StatusCode;
use serde;
use std::fmt::Error as StdError;

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    async fn request<T>(&self, url: String, auth_header: String, auth: String) -> Result<(), Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let resp = client.post(url).header(auth_header, auth).send().await?;

        if resp.status() == StatusCode::OK {
            let response = resp.json::<T>().await;
            // todo: use json pointer stored in database to get data
        }

        Ok(())
    }

    pub async fn build_url(&self, agency_id: i32) -> Result<String, Error> {
        let result = sqlx::query!(r#"SELECT api_url FROM agencies WHERE id = $1"#, agency_id)
            .fetch_one(&self.client.pool)
            .await?;
        let full_url = format!("https://{}", result.api_url);
        Ok(full_url)
    }

    /// Get's the agency's API secret from the environment as specified by key_env_name in the database
    pub async fn get_auth(&self, agency_id: i32) -> Result<String, Error> {
        let result = sqlx::query!(
            r#"SELECT key_env_name FROM agencies WHERE id = $1"#,
            agency_id
        )
        .fetch_one(&self.client.pool)
        .await?;
        let api_secret = std::env::var(result.key_env_name.unwrap());
        Ok(api_secret.unwrap())
    }

    /// Checks the line cache for the line, if not then it will try to get it from the api
    pub async fn try_line(&self, agency_id: i32, line: String) -> Result<String, Error> {
        let result = sqlx::query!(r#"SELECT line_id FROM agency_line_cache WHERE agency_id = $1 AND LOWER(line_name) = LOWER($2)"#, agency_id, line.to_string()).fetch_one(&self.client.pool)
            .await?;
        let api_url = &self.build_url(agency_id);
        Ok(result.line_id) // TODO: This better work
    }

    /// Attempts to get station data from the given agency, line and station name
    pub async fn try_station(
        &self,
        agency_id: i32,
        line: String,
        station: String,
    ) -> Result<(), Error> {
        let line = self
            .try_line(agency_id, line)
            .await?;
        let endpoint_pointer = &self.client.get_json_pointer(agency_id, EndpointType::AllLines);
        Ok(())
    }
}
