use crate::{
    constants::{MESSAGE_WELCOME_LOG, VERSION},
    utils::logger::{Logger, LoggerActions},
};

// Initial logging
pub fn welcome_log() {
    let logger = Logger::new();
    let message = format!("{}{}", &MESSAGE_WELCOME_LOG, &VERSION);
    logger.info(&message);
}
