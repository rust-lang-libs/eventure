use colored::{ColoredString, Colorize};
use uuid::Uuid;

pub fn generate_id() -> String {
    String::from(&Uuid::new_v4().to_string()[..6])
}

pub fn color_string(text: &str) -> ColoredString {
    text.bold().green().italic().underline()
}