use colored::Colorize;

pub fn format_target(target: &str) -> String {
    format!("{}", target.bold().yellow())
}