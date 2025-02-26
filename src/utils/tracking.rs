use crate::Error;
use sqlx::types::chrono::{DateTime, Utc};

pub struct Tracking{
    pub id: i32,
    pub owner_id: i64,
    pub agency_id: i32,
    pub line_id: i32,
    pub stop_id: Option<i32>,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl Tracking{
    fn new(){

    }

    /// Gets an update for this tracking, by making the necessary API calls and sending a
    fn get_update() -> Result<(), Error>{

        Ok(())
    }
}