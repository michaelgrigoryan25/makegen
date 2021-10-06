use crate::constants::{MESSAGE_WELCOME_LOG, VERSION};
use colored::Colorize;

// Initial logging
pub fn welcome_log() {
    println!("{}", format!("{}{}", &MESSAGE_WELCOME_LOG, &VERSION).green().bold());
}
