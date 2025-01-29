use crate::get_secrets;
use crate::serenity::{User, UserId};
use crate::Error;
use sqlx::postgres::PgPool;

pub struct Client {
    pool: PgPool,
}

impl Client {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let secrets = get_secrets();
        let pool = PgPool::connect(&secrets.database_url).await?;
        Ok(Self { pool })
    }

    /// Adds a user, a.k.a 'Passenger' to the database
    pub async fn add_user(pool: &PgPool, user: User) -> Result<UserId, Error> {
        sqlx::query!(
            r#"INSERT INTO users(id) VALUES ( $1 )"#,
            user.id.get() as i32
        )
        .execute(pool)
        .await?;

        Ok(user.id)
    }
}
