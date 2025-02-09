use crate::{Context, Error};
use poise::CreateReply;
use sqlx::{self, query};
use sqlx::{Column, Row};
use std::fmt::Write;

/// Top level command for development commands. Owner only
#[poise::command(
    prefix_command,
    track_edits,
    owners_only,
    hide_in_help,
    subcommand_required,
    subcommands("stop", "sql")
)]
pub async fn dev(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Stops the bot by shutting down all shards
#[poise::command(
    prefix_command,
    track_edits,
    owners_only,
    hide_in_help,
    category = "Dev"
)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    if let Context::Prefix(ctx) = ctx {
        // this if statement has to be here because of the way rust works
        // if it doesn't know for a fact that it is Context::Prefix, it will not see .msg
        ctx.msg.react(ctx, '👋').await?;
    }
    println!("\n{} is shutting down all shards...\n", ctx.author().name);
    let shard_manager = ctx.framework().shard_manager().clone();
    shard_manager.shutdown_all().await;
    Ok(())
}

/// Runs an SQL query
#[poise::command(
    prefix_command,
    track_edits,
    owners_only,
    hide_in_help,
    category = "Dev"
)]
pub async fn sql(ctx: Context<'_>, #[rest] query: String) -> Result<(), Error> {
    let pool = &ctx.data().db.pool;
    let result = sqlx::query(query.as_str()).fetch_all(pool).await;
    match result {
        Ok(rows) => {
            let mut response = String::new();
            for row in rows {
                let mut row_str = String::new();
                for (i, column) in row.columns().iter().enumerate() {
                    let value = match row.try_get::<String, _>(i) {
                        Ok(v) => v,
                        Err(_) => "NULL".to_string(),
                    };
                    write!(row_str, "{}: {}, ", column.name(), value)?;
                }
                writeln!(response, "{}", row_str)?;
            }
            let reply =
                CreateReply::default().content(format!("Query results:\n```\n{}\n```", response));
            ctx.send(reply).await?;
        }
        Err(e) => {
            let reply = CreateReply::default().content(format!("Error executing query: {}", e));
            ctx.send(reply).await?;
        }
    }

    Ok(())
}

/// Tracking command testing, owner only for now
#[poise::command(
    slash_command,
    prefix_command,
    track_edits,
    owners_only,
    hide_in_help,
    subcommands("list")
)]
pub async fn track(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Shows list of tracked lines or subway stations
#[poise::command(prefix_command, track_edits, owners_only, hide_in_help)]
pub async fn list(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
