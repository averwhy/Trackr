use crate::{Context, Error};
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>, // the command argument (string)
) -> Result<(), Error> { // this is the result of the command (what happens when it's called)
    poise::builtins::help( // this help function is builtin to poise
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

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn track(
    ctx: Context<'_>,
    #[description = "The name of the transit agency to check the status of"]
    agency_name: String,
) -> Result<(), Error> {
    // access config from Data
    let config = &ctx.data().config;
    let agencies = &config.agencies;
    let agency_name_upper = agency_name.to_uppercase();
    // match agency with user input
    let Some(agency) = agencies.get(&agency_name_upper) else {
        poise::send_reply(ctx, poise::CreateReply::default().ephemeral(false)
        .content(format!("Could not find agency `{}`", agency_name))).await?;
        return Ok(());
    };

    poise::send_reply(ctx, poise::CreateReply::default().ephemeral(false).content
        (format!("Pretending to track {}\nVisit them at {}", agency.name, agency.url))
    ).await?;

    Ok(())
}