use poise::serenity_prelude as serenity;
use std::{sync::Arc, time::Duration};
mod utils;
use crate::utils::agencies::get_all as get_agencies;
use crate::utils::agencies::Agency;
use crate::utils::config::get as get_config;
use crate::utils::config::Config;
use crate::utils::secrets::get as get_secrets;

use crate::utils::apis::mbta::MBTA;

mod commands;
mod dev_commands;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// Custom user data passed to all command functions

pub struct Data {
    pub config: Config,
    pub agencies: Vec<Agency>,
    pub mbta: MBTA,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = get_config();
    let agencies = get_agencies();

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        // this is a list of all commands from the commands.rs file
        commands: vec![
            commands::help(), // Public commands \/
            commands::track(),
            commands::about(),
            dev_commands::dev(), // Dev commands \/
            dev_commands::mbta(),
        ], // dev only commands (subcommands are NOT included because then they would be runnable on their own)
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(config.prefix.clone()),
            // tracks messages that are edited within the last hour
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|_ctx| {
            Box::pin(async move {
                // if ctx.author().id == 123456789 {
                //     return Ok(false);
                // }
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    };
    let secrets = get_secrets();
    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    config,
                    agencies,
                    mbta: MBTA::new(secrets.mbta_api_key),
                })
            })
        })
        .options(options)
        .build();
    let token = secrets.token;
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
