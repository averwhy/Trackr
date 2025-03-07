use crate::get_secrets;
use crate::serenity::{User, UserId, Command};
use crate::Error;
use sqlx::postgres::PgPool;
use tracing::{span, Level};

pub struct Client {
    pub pool: PgPool,
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
        let pool = get_pool.expect("Failed to setup Postgres on both an internal Docker network, and externally. Check database url(s)");
        span!(Level::INFO, "Postgres connected successfully");
        Ok(Self { pool })
    }

    /// Adds a user, a.k.a 'Passenger' to the database
    pub async fn add_user(self: Self, user: User) -> Result<UserId, Error> {
        sqlx::query!(
            r#"INSERT INTO users(id) VALUES ( $1 )"#,
            user.id.get() as i32
        )
        .execute(&self.pool)
        .await?;

        Ok(user.id)
    }

    /// Gets a user, a.k.a 'Passenger' from the database
    pub async fn get_user(self: Self, user: User) -> Result<UserId, Error> {
        sqlx::query!(r#"SELECT * FROM users WHERE id = $1"#, user.id.get() as i32)
            .fetch_one(&self.pool)
            .await?;

        Ok(user.id)
    }

    pub async fn command_exists(&self, command: Command) -> Result<bool, Error>{
        sqlx::query!(r#"SELECT * FROM command_stats WHERE command_name = $1"#, command.name)
            .fetch_one(&self.pool)
            .await?;
        Ok(true)
    }

    /// Adds an use to a specified command
    pub async fn update_command(&self, command: Command) -> Result<(), Error> {
        self.command_exists(command.clone()).await.expect(&format!("Command '{}' does not exist", command.name.to_string()));
        sqlx::query!(r#"UPDATE command_stats SET command_count = (command_count + 1) WHERE command_name = $1"#, command.clone().name).execute(&self.pool).await?;
        Ok(())
    }
}
