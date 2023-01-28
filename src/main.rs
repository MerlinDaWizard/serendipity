mod commands;
mod events;
mod helpers;
mod checks;
mod config;
//use crate::events::listener;
use std::{env, sync::{Arc}};

use dotenv::dotenv;
use poise::{serenity_prelude::{self as serenity, UserId}, Framework, FrameworkOptions};
use reqwest::Client;
use songbird::SerenityInit;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    songbird: Arc<songbird::Songbird>,

    bot_start_time: std::time::Instant,
    bot_user_id: UserId,
    version: String,
    http_client: Client
}

#[derive(Debug)]
pub enum MusicErrors {
    NoUserChannel,
    NotInGuild,
    BotNotInChannel,
}

impl std::error::Error for MusicErrors {}

impl std::fmt::Display for MusicErrors {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
 }

#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?; // Buildin command provides a nice button menu
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Dotenv crate automatically loads environment variables specified in `.env` into the environment
    env_logger::init();
    let songbird = songbird::Songbird::serenity();
    let songbird_clone = songbird.clone();
    let token = env::var("DISCORD_TOKEN").expect("Fill in DISCORD_TOKEN at .env");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = Framework::new(
        FrameworkOptions {
            commands: vec![commands::play(), commands::hello(), commands::stats(), register(), commands::stop(), commands::skip(), commands::nowplaying(), commands::pause(), commands::resume(), commands::teams(), commands::seek()],
            listener: |event, framework, data| {
                Box::pin(events::listener(event, framework, data))
            },
            ..Default::default()
        },
        move |_ctx, _ready, _framework| {
            log::info!("Creating framework");
            Box::pin(async move { Ok(
                Data {
                    songbird: songbird_clone,
                    bot_start_time: std::time::Instant::now(),
                    bot_user_id: _ready.user.id,
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    http_client: reqwest::Client::new(),
                }
            )})
        }
    );

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .unwrap();

    log::info!("Running bot");
    client.start_autosharded().await.unwrap();
}
