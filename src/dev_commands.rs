use crate::{Context, Error};
use poise::CreateReply;
use sqlx::{self, TypeInfo};
use sqlx::{Column, Row};
use sqlx::types::chrono;
use chrono_humanize::HumanTime;
use std::fmt::Write;
use tracing::{Level, span};

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
    let author_name = ctx.author().name.clone();
    span!(Level::INFO, "{} is shutting down all shards- bye!\n", author_name);
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
                    let value: String = match column.type_info().name() {
                        "TEXT" | "VARCHAR" => row.try_get::<String, _>(i).unwrap_or("NULL".to_string()),
                        "INTEGER" | "INT" | "INT8" | "BIGINT" | "SMALLINT" => row.try_get::<i64, _>(i).map(|v| v.to_string()).unwrap_or("NULL".to_string()),
                        "REAL" | "FLOAT" | "DOUBLE" => row.try_get::<f64, _>(i).map(|v| v.to_string()).unwrap_or("NULL".to_string()),
                        "BOOLEAN" | "BOOL" => row.try_get::<bool, _>(i).map(|v| v.to_string()).unwrap_or("NULL".to_string()),
                        "TIMESTAMP" => row.try_get::<chrono::DateTime<chrono::Utc>, _>(i).map(|v| HumanTime::from(v).to_string()).unwrap_or("NULL".to_string()),
                        _ => "Unsupported Type".to_string(),
                    };
                    write!(row_str, "{} ({}): {}, ", column.name(), column.type_info().name(), value)?;
                }
                writeln!(response, "{}", row_str)?;
            }
            if response.chars().count() <= 0 {
                // There was no result, lets just react with a check
                if let Context::Prefix(ctx) = ctx {
                    ctx.msg.react(ctx, '✅').await?;
                }
            }
            let reply =
                CreateReply::default().content(format!("Query results:\n```\n{}\n```", response));
            ctx.send(reply).await?;
        }
        Err(e) => {
            let reply = CreateReply::default().content(format!("{}", e));
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
