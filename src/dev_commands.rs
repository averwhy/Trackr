use crate::{Context, Error};
use poise::CreateReply;
use serenity::builder::CreateEmbed;
use sqlx::types::chrono;
use sqlx::{self, TypeInfo};
use sqlx::{Column, Row};
use tracing::{span, Level};

use crate::utils::modals;

/// Top level command for development commands. Owner only
#[poise::command(
    prefix_command,
    track_edits,
    owners_only,
    hide_in_help,
    subcommand_required,
    subcommands("stop", "sql", "addagency")
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
    span!(Level::INFO, "{} is shutting down all shards", author_name);
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
            let mut embed = CreateEmbed::new();
            for row in &rows {
                for (i, column) in row.columns().iter().enumerate() {
                    let value: String = match column.type_info().name() {
                        "TEXT" | "VARCHAR" => {
                            row.try_get::<String, _>(i).unwrap_or("NULL".to_string())
                        }
                        "INTEGER" | "INT" | "INT8" | "BIGINT" | "SMALLINT" => row
                            .try_get::<i64, _>(i)
                            .map(|v| v.to_string())
                            .unwrap_or("NULL".to_string()),
                        "REAL" | "FLOAT" | "DOUBLE" => row
                            .try_get::<f64, _>(i)
                            .map(|v| v.to_string())
                            .unwrap_or("NULL".to_string()),
                        "BOOLEAN" | "BOOL" => row
                            .try_get::<bool, _>(i)
                            .map(|v| v.to_string())
                            .unwrap_or("NULL".to_string()),
                        "TIMESTAMPTZ" => row
                            .try_get::<chrono::DateTime<chrono::Utc>, _>(i)
                            .map(|v| format!("<t:{}:R>", v.timestamp()))
                            .unwrap_or("NULL".to_string()),
                        _ => "Unsupported Type".to_string(),
                    };
                    embed = embed.clone().field(column.name(), value, false);
                }
            }
            if rows.len() <= 0usize {
                // There was no result, lets just react with a check
                if let Context::Prefix(ctx) = ctx {
                    ctx.msg.react(ctx, '✅').await?;
                }
            } else {
                let reply = CreateReply::default().embed(embed);
                ctx.send(reply).await?;
            }
        }
        Err(e) => {
            let reply = CreateReply::default().content(format!("{}", e));
            ctx.send(reply).await?;
        }
    }

    Ok(())
}

/// Adds a transit agency to the database
#[poise::command(prefix_command, owners_only, hide_in_help, category = "Dev")]
pub async fn addagency(_ctx: Context<'_>) -> Result<(), Error> {
    // When we add an agency, we need to add something to the following tables:
    // agencies, endpoints, and endpoint_pointers
    // We made 3 modals for those tables in utils/modals.rs
    // We'll send a message with a button to start that process
    let initial_embed = CreateEmbed::new()
        .title("Agency Addition Tool")
        .description("Click the button below to start adding an agency.\nEach form/component/button etc. has a timeout of 1 hour.");

    let start_button = poise::serenity_prelude::CreateButton::new("add_agency_info_button")
        .label("Add Agency Info")
        .style(poise::serenity_prelude::ButtonStyle::Success);

    //let components = poise::serenity_prelude::CreateActionRow::Buttons(); // todo figure this out

    let initial_reply = CreateReply::default()
        .embed(initial_embed);
        //.components(start_button); // todo figure that out

    
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
pub async fn track(
    ctx: Context<'_>,
    agency: String,
    line: String,
    station: String,
) -> Result<(), Error> {
    // Will try to get the agency in the DB based off the agency name passed in, e.g. /track mbta
    let agency_id = ctx.data().db.get_agency(agency).await?;
    // Now try to see if we can find the line that they want based on the agency (e.g. red line)
    let line_id = ctx.data().api.try_line(agency_id, line).await?;
    // Now try to get the station
    Ok(())
}

/// Shows list of tracked lines or subway stations
#[poise::command(prefix_command, track_edits, owners_only, hide_in_help)] // TODO: Once done, add back slash command support
pub async fn list(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Gets the ETA of the next train at a specified station
#[poise::command(prefix_command, track_edits, owners_only, hide_in_help)] // TODO: Once done, add back slash command support
pub async fn eta(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
