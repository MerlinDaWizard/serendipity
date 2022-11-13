use std::{env, time::Instant};

use dotenv::dotenv;

use poise::serenity_prelude::{self as serenity, Mention};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct Data {
    bot_start_time: Instant
}

#[poise::command(slash_command)]
async fn hello(
    ctx: Context<'_>, // <'_> means an anonomous lifetime
    #[description = "Selected user"] user: serenity::User, // Here description is text attached to the command arguement description
) -> Result<(), Error> { // Returns an empty tuple, meaning success (see ok below) or an error
    ctx.say(format!("{} says hello to {} :D", Mention::from(ctx.author().id), Mention::from(user.id))).await?;
    Ok(())
}

#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?; // Buildin command provides a nice button menu
    Ok(())
}

const DELIMETER: char = '・';
struct TimeData {
    years: u16,
    days: u16,
    hours: u8,
    mins: u8,
    secs: u8,
}

const YEAR: u64 = 365*24*60*60; // Yes we ignore leap years and stuff, sue me
const DAY: u64 = 24*60*60; // Stored as u64 just to keep arithmetic kinda simple
const HOUR: u64 = 60*60;
const MINUTE: u64 = 60;

impl TimeData {
    fn new(time_delta: u64) -> TimeData {
        TimeData {
            years: (time_delta / YEAR) as u16,
            days: (time_delta % YEAR / DAY) as u16,
            hours: (time_delta % DAY / HOUR) as u8,
            mins: (time_delta % HOUR / MINUTE) as u8,
            secs: (time_delta % MINUTE) as u8,
        }
    }

    fn format(&self) -> String {
        let mut formatted = String::new();
        let mut after = false;
        if self.years != 0 {
            formatted.push_str(&format!("{} Yrs", self.years));
            after = true;
        }
        if after || self.days != 0 {
            if after {formatted.push(DELIMETER)} else {after = true}
            formatted.push_str(&format!("{} Days", self.days));
        }
        if after || self.hours != 0 {
            if after {formatted.push(DELIMETER)} else {after = true}
            formatted.push_str(&format!("{} Hrs", self.hours));        }
        if after || self.mins != 0 {
            if after {formatted.push(DELIMETER)} else {after = true}
            formatted.push_str(&format!("{} Mins", self.mins));
        }
        if after || self.secs != 0 {
            if after {formatted.push(DELIMETER)}
            formatted.push_str(&format!("{} Secs", self.secs))
        }
        return formatted;
    }
}

async fn get_system_uptime() -> String {
    match uptime_lib::get() {
        Ok(uptime) => {
           return TimeData::new(uptime.as_secs()).format();
        }
        Err(err) => {
            eprintln!("Error getting uptime: {}", err);
            return "Err".to_string();
        }
    }
}

/// Returns some system stats and uptime data
#[poise::command(slash_command)]
async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    let uptime = Instant::now() - ctx.data().bot_start_time;
    ctx.say(TimeData::new(uptime.as_secs()).format()).await?;
    ctx.say(get_system_uptime().await).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Dotenv crate automatically loads environment variables specified in `.env` into the environment
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![stats(), hello(), register()], // We specify the commands in an 'array' (vec in rust), we then load the default values for the framework for the rest
            ..Default::default()
        })
        // Login with a bot token from the environment
        .token(env::var("DISCORD_TOKEN").expect("Fill in DISCORD_TOKEN at .env")) // .expect means we just panic (crash kinda) if its missing
        // Use a bitwise OR to add the message context intent, due to intents stored as 53-bit integer bitfield
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(
            Data {
                bot_start_time: std::time::Instant::now()
            }
        )}));
        
        framework.run().await.unwrap();
}
