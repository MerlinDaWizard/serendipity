mod commands;
use std::{env};

use dotenv::dotenv;
use env_logger::Env;
use poise::{serenity_prelude::{self as serenity,EventHandler, UserId, Ready}, async_trait, Framework, event::EventWrapper};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
 }

pub struct Data {
    bot_start_time: std::time::Instant,
    bot_user_id: UserId,
    version: String,
}

#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?; // Buildin command provides a nice button menu
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Dotenv crate automatically loads environment variables specified in `.env` into the environment
    //let env = Env::new().filter("RUST_LOG");
    let mut builder = env_logger::Builder::new();
    todo!();
    builder.target(env_logger::Target::Stdout); // TODO: NOT CURRENTLY WORKING
    //builder.filter_level(log::LevelFilter::Info);
    builder.filter_module("main", log::LevelFilter::Info);
    builder.init();
    //env_logger::init();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::hello(), commands::stats(), register()], // We specify the commands in an 'array' (vec in rust), we then load the default values for the framework for the rest
            event_handler: |ctx, event, framework, data| {
				Box::pin(listener(ctx, event, framework, data))
			},
            ..Default::default()
        })
        // Login with a bot token from the environment
        .token(env::var("DISCORD_TOKEN").expect("Fill in DISCORD_TOKEN at .env")) // .expect means we just panic (crash kinda) if its missing
        // Use a bitwise OR to add the message context intent, due to intents stored as 53-bit integer bitfield
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |_ctx, _ready, _framework| {
            log::info!("Loading");
            log::error!("Test");
            Box::pin(async move { Ok(
                Data {
                    bot_start_time: std::time::Instant::now(),
                    bot_user_id: _ready.user.id,
                    version: env!("CARGO_PKG_VERSION").to_string(),
                }
            )})
        });
    /////////////////////
    framework.run_autosharded().await.unwrap();
}

async fn listener(
	ctx: &serenity::Context,
	event: &poise::Event<'_>,
	framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("{} is connected!",data_about_bot.user.name)
        }
        _ => {}
    }
    //println!("{:?}",event);
    Ok(())
}