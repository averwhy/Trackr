use crate::{Context, Error};
const VERSION: &str = env!("CARGO_PKG_VERSION");
#[allow(dead_code)]
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

use crate::utils::agencies::get as get_agency;

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
{} Track subways, busses, regional rail, and more.
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

/// Track a transit agency
#[poise::command(prefix_command, track_edits, slash_command, category = "Tracking")]
pub async fn track(
    ctx: Context<'_>,
    #[description = "The name of the transit agency to check the status of"] agency_name: String,
) -> Result<(), Error> {
    let agency_name_upper = agency_name.to_uppercase();
    let Some(agency) = get_agency(agency_name_upper) else {
        poise::send_reply(
            ctx,
            poise::CreateReply::default()
                .ephemeral(false)
                .content(format!("Could not find agency `{}`", agency_name)),
        )
        .await?;
        return Ok(());
    };
    poise::send_reply(
        ctx,
        poise::CreateReply::default()
            .ephemeral(false)
            .content(format!(
                "Pretending to track {}\nVisit them at {}",
                agency.name, agency.url
            )),
    )
    .await?;
    Ok(())
}
