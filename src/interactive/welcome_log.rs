use crate::constants::{VERSION, WELCOME_MESSAGE};

// Initial logging
pub fn welcome_log() {
    println!(
        "{}",
        format!(
            r"
            {welcome}{version}
            ",
            welcome = &WELCOME_MESSAGE,
            version = &VERSION
        )
        .trim()
    );
}
