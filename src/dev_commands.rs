use crate::{Context, Error};
use poise::CreateReply;

/// Top level command for development commands. Owner only
#[poise::command(
    prefix_command,
    track_edits,
    owners_only,
    hide_in_help,
    subcommand_required,
    subcommands("stop")
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

/// MBTA API Commands testing. owner only
#[poise::command(
    prefix_command,
    track_edits,
    owners_only,
    hide_in_help,
    subcommand_required,
    subcommands("alerts", "lines")
)]
pub async fn mbta(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Gets alerts data from the MBTA API
#[poise::command(prefix_command, track_edits, owners_only, hide_in_help)]
pub async fn alerts(ctx: Context<'_>) -> Result<(), Error> {
    let alerts = ctx.data().mbta.get_alerts().await?;
    let subway_alert_titles: Vec<String> = alerts
        .subway_delays
        .iter()
        .map(|alert| {
            alert
                .attributes
                .as_ref()
                .unwrap()
                .short_header
                .as_ref()
                .unwrap()
        })
        .cloned()
        .collect();
    ctx.send(
        CreateReply::default().content(format!("Number of alerts: {}\nAlert titles: {}", alerts.subway_delay_count, subway_alert_titles.join("\n"))),
    )
    .await?;
    Ok(())
}

/// Gets lines data from the MBTA API
#[poise::command(prefix_command, track_edits, owners_only, hide_in_help)]
pub async fn lines(ctx: Context<'_>) -> Result<(), Error> {
    let lines = ctx.data().mbta.get_lines().await?;
    let subways = lines.subways;
    let subway_names: Vec<String> = subways
        .iter()
        .map(|subway| subway.attributes.long_name.clone())
        .collect();
    ctx.send(CreateReply::default().content(format!(
        "Number of lines: {}\nLong names: {}",
        lines.count,
        subway_names.join(", ")
    )))
    .await?;
    Ok(())
}
