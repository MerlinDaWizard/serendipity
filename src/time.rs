use std::time::Duration;

const YEAR: u64 = 365*24*60*60; // Yes we ignore leap years and stuff, sue me
const DAY: u64 = 24*60*60; // Stored as u64 just to keep arithmetic kinda simple
const HOUR: u64 = 60*60;
const MINUTE: u64 = 60;

const DELIMETER: char = '・';

/// A struct used to format durations easily, ignores anything sub seconds.
pub struct DurationFormatter {
    years: u16,
    days: u16,
    hours: u8,
    mins: u8,
    secs: u8,
}

impl DurationFormatter {
    pub fn new(duration: Duration) -> DurationFormatter {
        let time_delta_seconds = duration.as_secs();
        DurationFormatter {
            years: (time_delta_seconds / YEAR) as u16,
            days: (time_delta_seconds % YEAR / DAY) as u16,
            hours: (time_delta_seconds % DAY / HOUR) as u8,
            mins: (time_delta_seconds % HOUR / MINUTE) as u8,
            secs: (time_delta_seconds % MINUTE) as u8,
        }
    }

    pub fn format_long(&self) -> String {
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
        formatted
    }

    pub fn format_short(&self) -> String {
        let mut formatted = String::new();
        let mut atleast_one = false;
        if (self.days + self.years * 365) != 0 {
            formatted = formatted + &format!("{}d ", (self.days + self.years * 365));
            atleast_one = true;
        }

        if self.hours != 0 {
            formatted = formatted + &format!("{}h ", self.hours);
            atleast_one = true;
        }

        if self.mins != 0 {
            formatted = formatted + &format!("{}m ", self.mins);
            atleast_one = true;
        }

        if !atleast_one || self.secs != 0 {
            formatted = formatted + &format!("{}s ", self.secs);
        }

        // Remove the last space to keep things compact
        if let Some(i) = formatted.rfind(' ') {
            formatted.remove(i);
        };

        formatted
    }
}