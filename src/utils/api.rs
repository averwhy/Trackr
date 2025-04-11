use crate::utils::database::Client;
use crate::utils::database::EndpointType;
use crate::Error;
use jsonptr::Pointer;
use reqwest::StatusCode;
use serde;
use serenity::json;
use sqlx::Value;

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    async fn request(
        &self,
        agency_id: i32,
        json_pointer: String,
        auth_header: String,
    ) -> Result<serde_json::Value, Error> {
        let url = &self.get_url(agency_id).await?;
        let auth = &self.get_auth(agency_id).await?;

        let client = reqwest::Client::new();
        let resp = client.post(url).header(auth_header, auth).send().await?;

        let status = resp.status();
        let response = resp.text().await?;
        let json_value = serde_json::from_str::<serde_json::Value>(&response)?;
        if status == StatusCode::OK {
            let ptr = Pointer::parse(&json_pointer)?;
            let data = ptr.resolve(&json_value)?;
            Ok(data.clone())
        } else {
            Ok(json_value)
        }
    }

    async fn get_url(&self, agency_id: i32) -> Result<String, Error> {
        let result = sqlx::query!(r#"SELECT api_url FROM agencies WHERE id = $1"#, agency_id)
            .fetch_one(&self.client.pool)
            .await?;
        let full_url = format!("https://{}", result.api_url);
        Ok(full_url)
    }

    /// Get's the agency's API secret from the environment as specified by key_env_name in the database
    async fn get_auth(&self, agency_id: i32) -> Result<String, Error> {
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
        //let api_url = &self.request(agency_id);
        Ok(result.line_id) // TODO: This better work
    }

    /// Attempts to get station data from the given agency, line and station name
    pub async fn try_station(
        &self,
        agency_id: i32,
        line: String,
        station: String,
    ) -> Result<(), Error> {
        let line_result = self.try_line(agency_id, line).await?;
        let endpoint_pointer = self
            .client
            .get_json_pointer(agency_id, EndpointType::AllStations)
            .await?;
        let auth_header = &self.client.get_agency_auth_header(agency_id).await?;

        let result = self
            .request(
                agency_id,
                endpoint_pointer.pointer_path,
                auth_header.clone(),
            )
            .await?;

        Ok(())
    }
}
