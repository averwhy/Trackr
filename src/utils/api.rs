use crate::serenity::{Command, User, UserId};
use crate::utils::database::Client;
use crate::Error;
use sqlx::Row;

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn build_url(&self, agency_id: i32) -> Result<String, Error> {
        let result = sqlx::query!(
            r#"SELECT api_url FROM agencies WHERE id = $1"#,
            agency_id
        )
        .fetch_one(&self.client.pool)
        .await?;
        let full_url = format!("https://{}", result.api_url);
        Ok(full_url)
    }

    /// Checks the line cache for the line, if not then it will try to get it from the api
    pub async fn try_line(&self, agency_id: i32, line: String) -> Result<String, Error> {
        let line_id = sqlx::query!(r#"SELECT line_id FROM agency_line_cache WHERE agency_id = $1 AND LOWER(line_name) = LOWER($2)"#, agency_id, line.to_string()).fetch_one(&self.client.pool)
            .await?;
        let api_url = &self.build_url(agency_id);
        Ok(line_id.line_id) // TODO: This better work
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
            .await
            .expect("Could not find any matching lines");
        // TODO: api call to get station, return results
        Ok(())
    }
}
