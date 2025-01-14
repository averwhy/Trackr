#![warn(clippy::str_to_string)]

mod commands;

use poise::serenity_prelude as serenity;
use std::{
    collections::HashMap,
    env::var,
    sync::Arc,
    time::Duration,
};

// get json data from config.json
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
  pub prefix: String,
  pub agencies: HashMap<String, Agency>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Agency{
    pub name: String,
    pub url: String,
    pub api_url: String
}
fn get_config() -> Config {
    let file = File::open("config.json").expect("file should open read only");
    return serde_json::from_reader(file).expect("file should be proper JSON")
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Secrets {
    token: String
}
fn get_secrets() -> Secrets {
    let file = File::open("secrets.json").expect("file should open read only");
    return serde_json::from_reader(file).expect("file should be proper JSON")
}

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    pub config: Config,
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

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        // this is a list of all commands from the commands.rs file
        commands: vec![commands::help(), commands::track()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(config.prefix.clone()),
            // tracks messages that are edited within the last hour
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                // funny alternate prefixes that are pointless
                poise::Prefix::Literal("hey trackr,"),
                poise::Prefix::Literal("hey trackr"),
            ],
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
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
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

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    config,
                })
            })
        })
        .options(options)
        .build();
    
    let secrets = get_secrets();
    let token = secrets.token;
    let intents =
        serenity::GatewayIntents::non_privileged();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}