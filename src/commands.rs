use crate::{Context, Error};
const VERSION: &str = env!("CARGO_PKG_VERSION");
#[allow(dead_code)]
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command, category = "Misc")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "The command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>, // the command argument (string)
) -> Result<(), Error> {
    // this is the result of the command (what happens when it's called)
    poise::builtins::help(
        // this help function is builtin to poise
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: &format!("Tracker v{VERSION}"),
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Information about Trackr
#[poise::command(prefix_command, track_edits, slash_command, category = "Misc")]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let about: String = format!("Trackr v{}, created by @averwhy
{} Track subways, regional rail, and more.
*Please note that Trackr is under active development and is not fully functional.*
The current scope of the bot is: ```md
<On Release (v1.0.0)>
- Check status of transit agencies (regional rail, subway/lrt, brt)
- When a command like '/track mbta red' is ran, it will reply with an embed that will be regularly updated with the red lines status. it would also work with like '/track mbta orange NorthStation', except with stations it would show the expected arrival times for that line
<After release/in the future>
- Ability to track Amtrak lines
- Ability to track individual amtrak trains, and recieve notifications about train updates/station arrivals
```
For a list of available commands, type `/help` or `=help` (both text and slash commands are supported)."
    , VERSION, DESCRIPTION);
    poise::send_reply(ctx, poise::CreateReply::default().content(about)).await?;
    Ok(())
}

/// Shows server count
#[poise::command(prefix_command, category = "Misc")]
pub async fn servers(ctx: Context<'_>) -> Result<(), Error> {
    let guilds = ctx.cache().guilds().len();
    poise::send_reply(
        ctx,
        poise::CreateReply::default().content(format!("I'm in {guilds} servers!")),
    )
    .await?;
    Ok(())
}

/// Support server link
#[poise::command(prefix_command, slash_command, category = "Misc")]
pub async fn support(ctx: Context<'_>) -> Result<(), Error> {
    poise::send_reply(
        ctx,
        poise::CreateReply::default().content(
            "For support, ideas, foaming, and more, join our support server: discord.gg/kzZJ87WMEQ",
        ),
    )
    .await?;
    Ok(())
}
