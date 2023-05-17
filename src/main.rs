#[macro_use]
extern crate log;

pub mod commands;
pub mod config;
pub mod events;

use std::sync::Arc;

use config::Config;
use dashmap::DashMap;
use poise::{Framework, FrameworkOptions};
use serenity::{
    all::{GuildId, UserId},
    prelude::GatewayIntents,
};
use songbird::{SerenityInit, Songbird};
use tokio::time::Instant;

use crate::config::load_config;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let config = load_config("config.ron".into());
    pretty_env_logger::init();

    let songbird = songbird::Songbird::serenity();
    let songbird_clone = songbird.clone();
    let token = config.bot_token.clone();
    let intents = GatewayIntents::non_privileged();

    let framework = Framework::new(
        FrameworkOptions {
            commands: vec![register(), commands::join()],
            listener: |event, framework, data| Box::pin(events::listener(event, framework, data)),
            ..Default::default()
        },
        move |_ctx, ready, _framework| {
            info!("Creating framework");
            Box::pin(async move {
                Ok(Data {
                    config,
                    bot_user_id: ready.user.id,
                    songbird: songbird_clone,
                    http_client: reqwest::Client::new(),
                    timeouts: Arc::new(DashMap::new()),
                })
            })
        },
    );

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .unwrap();

    info!("Running bot");
    client.start_autosharded().await.unwrap();
}

#[derive(Debug)]
pub struct Data {
    config: Config,
    bot_user_id: UserId,
    songbird: Arc<Songbird>,
    http_client: reqwest::Client,
    timeouts: Arc<DashMap<GuildId, Timeouts>>,
}

#[derive(Debug, Default)]
pub struct Timeouts {
    lonely_leavetime: Option<Instant>,
    idle_leavetime: Option<Instant>,
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    // let data = ctx.data();
    // let spotify = &data.config.spotify_settings;
    // let client = BasicClient::new(
    //     ClientId::new(spotify.client_id.clone()),
    //     Some(ClientSecret::new(spotify.client_secret.clone())),
    //     AuthUrl::new(spotify.access_token_url.clone()).unwrap(),
    //     // Some(TokenUrl::new(spotify.access_token_url.clone()).unwrap()),
    //     Some(
    //         TokenUrl::new(
    //             "https://api.spotify.com/v1/albums/7JR7tGOAvqFSpVmDlCzHIJ/tracks".to_string(),
    //         )
    //         .unwrap(),
    //     ),
    // );

    // let cred = client.exchange_client_credentials();
    // let req = cred
    //     .request_async(oauth2::reqwest::async_http_client)
    //     .await
    //     .unwrap();

    // dbg!(req);

    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
