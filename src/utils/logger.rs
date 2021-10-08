use colored::Colorize;

#[derive(Debug, Clone, Default)]
pub struct Logger;

pub trait LoggerActions {
    fn log(&self, message: &str);
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
    fn success(&self, message: &str);
}

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }
}

impl LoggerActions for Logger {
    fn log(&self, message: &str) {
        println!("{}", &message.bold());
    }

    fn info(&self, message: &str) {
        println!("{}", &message.blue().bold());
    }

    fn warn(&self, message: &str) {
        println!("{}", &message.yellow().bold());
    }

    fn error(&self, message: &str) {
        println!("{}", &message.red().bold());
    }

    fn success(&self, message: &str) {
        println!("{}", &message.green().bold());
    }
}
