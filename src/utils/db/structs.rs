use sqlx;
use sqlx::types::chrono;

#[derive(sqlx::FromRow)]
pub struct Users {
    #[sqlx(rename = "id")]
    pub user_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
#[derive(sqlx::FromRow)]
pub struct UserStats {
    pub user_id: i32,
    #[sqlx(default)]
    pub checked_count: Option<i32>,
    #[sqlx(default)]
    pub tracked_count: Option<i32>,
    pub last_checked: chrono::DateTime<chrono::Utc>,
    pub last_tracked: chrono::DateTime<chrono::Utc>,
    pub last_alert: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
pub struct CommandStats {
    pub id: i32,
    pub command_name: String,
    pub command_count: i32,
    pub last_run: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
pub struct ActiveTrackings {
    pub id: i32,
    pub user_id: i32,
    pub agency_id: i32,
    pub line_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
pub struct Agencies {
    pub id: i32,
    pub short_name: String,
    pub long_name: String,
    pub api_url: String,
    pub key_required: bool,
    pub key_env_name: String,
    pub auth_header_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
pub struct AgencyLines {
    pub id: i32,
    pub agency_id: i32,
    pub line_id: String,
    pub line_name: String,
    pub line_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>
}

#[derive(sqlx::FromRow)]
pub struct Endpoints {
    pub id: i32,
    pub agency_id: i32,
    pub endpoint_type: String,
    pub endpoint_path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>
}

#[derive(sqlx::FromRow)]
pub struct EnpointPointers{
    pub id: i32,
    pub endpoint_id: i32,
    pub pointer_key: String,
    pub pointer_path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>
}