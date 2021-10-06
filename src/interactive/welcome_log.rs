use crate::constants::{VERSION, WELCOME_MESSAGE};
use colored::Colorize;

// Initial logging
pub fn welcome_log() {
    println!(
        "{}",
        format!("{}{}", &WELCOME_MESSAGE, &VERSION).green().bold()
    );
}
