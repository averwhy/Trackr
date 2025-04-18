use crate::get_secrets;
use crate::serenity::{Command, User, UserId};
use crate::Error;
use sqlx::postgres::PgPool;
use sqlx::types::chrono::{DateTime, Utc};
use tracing::{span, Level};

pub enum EndpointType {
    AllAlerts,
    Alert,
    AllLines,
    Lines,
    AllStations,
    Station,
    StopPrediction,
}

impl EndpointType {
    fn as_str(&self) -> &'static str {
        match self {
            EndpointType::AllAlerts => "ALL_ALERTS",
            EndpointType::Alert => "ALERT",
            EndpointType::AllLines => "ALL_LINES",
            EndpointType::Lines => "LINES",
            EndpointType::AllStations => "ALL_STATIONS",
            EndpointType::Station => "STATION",
            EndpointType::StopPrediction => "PREDICTION",
        }
    }
}

pub struct Client {
    pub pool: PgPool,
}
pub struct DbPassenger {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>, // This should not be optional but whatever i guess
}

pub struct DbEndpointPointer {
    pub id: i32,
    pub endpoint_id: i32,
    pub pointer_key: String,
    pub pointer_path: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Client {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let secrets = get_secrets();
        let get_pool: Result<PgPool, Error> = async {
            match PgPool::connect(&secrets.database_url).await {
                Ok(pool) => Ok(pool),
                Err(e) => PgPool::connect(&secrets.alt_database_url)
                    .await
                    .map_err(|_| e),
            }
            .map_err(|e| e.into())
        }
        .await;
        let pool = get_pool.expect("Failed to find Postgres on both the internal Docker network, and external network. Check database url(s)");
        span!(Level::INFO, "Postgres connected successfully");
        Ok(Self { pool })
    }

    /// Adds a user, a.k.a 'Passenger' to the database
    pub async fn add_user(&self, user: User) -> Result<UserId, Error> {
        sqlx::query!(
            r#"INSERT INTO users(id) VALUES ( $1 )"#,
            user.id.get() as i32
        )
        .execute(&self.pool)
        .await?;

        Ok(user.id)
    }

    /// Gets a user, a.k.a 'Passenger' from the database
    pub async fn get_user(&self, user: User) -> Result<DbPassenger, Error> {
        let result = sqlx::query_as!(
            DbPassenger,
            r#"SELECT * FROM users WHERE id = $1"#,
            user.id.get() as i32
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    /// Gets an Agency ID by checking short or long name
    pub async fn get_agency(&self, agency: String) -> Result<i32, Error> {
        let result = sqlx::query!(
            r#"SELECT id FROM agencies WHERE LOWER(short_name) = LOWER($1) OR LOWER(long_name) = LOWER($1)"#,
            agency
        )
            .fetch_one(&self.pool)
            .await?;
        Ok(result.id)
    }

    /// Gets an agency's auth header name for making requests
    pub async fn get_agency_auth_header(&self, agency_id: i32) -> Result<String, Error> {
        let result = sqlx::query!(
            r#"SELECT auth_header_name FROM agencies WHERE id = $1"#,
            agency_id
        )
        .fetch_one(&self.pool)
        .await?;

        result
            .auth_header_name
            .ok_or_else(|| Error::from("Agency has no auth header name"))
    }

    pub async fn get_json_pointer(
        &self,
        agency_id: i32,
        endpoint_type: EndpointType,
    ) -> Result<DbEndpointPointer, Error> {
        let endpoint_result = sqlx::query!(
            r#"SELECT * FROM endpoints WHERE agency_id = $1 AND endpoint_type = $2"#,
            agency_id,
            endpoint_type.as_str()
        )
        .fetch_one(&self.pool)
        .await?;

        let pointer_result = sqlx::query_as!(
            DbEndpointPointer,
            r#"SELECT * FROM endpoint_pointers WHERE endpoint_id = $1"#,
            endpoint_result.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(pointer_result)
    }

    pub async fn command_exists(&self, command: Command) -> Result<bool, Error> {
        sqlx::query!(
            r#"SELECT * FROM command_stats WHERE command_name = $1"#,
            command.name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(true)
    }

    /// Adds an use to a specified command
    pub async fn update_command(&self, command: Command) -> Result<(), Error> {
        self.command_exists(command.clone()).await.expect(&format!(
            "Command '{}' does not exist",
            command.name.to_string()
        ));
        sqlx::query!(r#"UPDATE command_stats SET command_count = (command_count + 1) WHERE command_name = $1"#, command.clone().name).execute(&self.pool).await?;
        Ok(())
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}
