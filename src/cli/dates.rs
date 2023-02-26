use chrono::NaiveDateTime;
use colored::Colorize;

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
