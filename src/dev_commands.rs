use crate::{Context, Error};

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
#[poise::command(prefix_command, track_edits, owners_only, hide_in_help, category = "Dev")]
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
