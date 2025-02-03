use crate::{Context, Error};
use poise::CreateReply;
use sqlx::{self, query};
use tracing_subscriber::fmt::format;
use sqlx::Row;

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
    let result = "";
    // todo for this: write structs for all db tables
    if query.to_uppercase().contains("SELECT"){
        let r = sqlx::query(
            query.as_str()
        )
        .fetch_one(&ctx.data().db.pool)
        .await
        .map_err(|e| format!("Query execution failed: {}", e))?;
    } else {
        sqlx::query(
            query.as_str()
        )
        .execute(&ctx.data().db.pool)
        .await
        .map_err(|e| format!("Query execution failed: {}", e))?;
    }
    poise::send_reply(ctx, poise::CreateReply::default().content(result)).await?;
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
