use poise::serenity_prelude::{self as serenity};
use crate::{Context, Error};

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
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    let uptime = std::time::Instant::now() - ctx.data().bot_start_time;
    ctx.say(TimeData::new(uptime.as_secs()).format()).await?;
    ctx.say(get_system_uptime().await).await?;
    Ok(())
}
