use chrono::{Local, NaiveDateTime};
use colored::{ColoredString, Colorize};

pub fn parse_fuzzy_date(date_string: Option<String>) -> Option<NaiveDateTime> {
    if let Some(date_string) = date_string {
        match fuzzydate::parse(date_string) {
            Ok(date) => Some(date),
            Err(err) => panic!("{} {:?}", "error:".red().bold(), err),
        }
    } else {
        None
    }
}

pub fn date_as_string(date: &Option<NaiveDateTime>) -> ColoredString {
    if date.is_some() {
        let date = date.unwrap().date();
        let date_string = format!("{}", date.format("%Y-%m-%d"));
        let now = Local::now().date_naive();

        if date <= now {
            // If the date is today or past today
            date_string.bright_red()
        } else if now.succ_opt().unwrap() == date {
            // If the date is tomorrow
            date_string.yellow()
        } else {
            // Otherwise the date is too far in the past
            date_string.white()
        }
    } else {
        "N/A".bright_black()
    }
}
