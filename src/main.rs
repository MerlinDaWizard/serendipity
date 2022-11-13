use std::env;

use dotenv::dotenv;

use poise::serenity_prelude::{self as serenity, Mention};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct Data{}

#[poise::command(slash_command, prefix_command)] // This is a function attibute from the poise library
async fn hello(
    ctx: Context<'_>, // <'_> means an anonomous lifetime
    #[description = "Selected user"] #[rest] user: serenity::User, // Here description is text attached to the command arguement description
) -> Result<(), Error> { // Returns an empty tuple, meaning success (see ok below) or an error
    ctx.say(format!("{} says hello to {} :D", Mention::from(ctx.author().id), Mention::from(user.id))).await?;
    return Ok(());
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?; // Buildin command provides a nice button menu
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Dotenv crate automatically loads environment variables specified in `.env` into the environment
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), register()], // We specify the commands in an 'array' (vec in rust), we then load the default values for the framework for the rest
            ..Default::default()
        })
        // Login with a bot token from the environment
        .token(env::var("DISCORD_TOKEN").expect("Fill in DISCORD_TOKEN at .env")) // .expect means we just panic (crash kinda) if its missing
        // Use a bitwise OR to add the message context intent, due to intents stored as 53-bit integer bitfield
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {} )}));
        
        framework.run().await.unwrap();
}
