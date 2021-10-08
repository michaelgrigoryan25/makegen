use crate::constants::ERROR_MAKEFILE_EXISTS;
use filesystem::FileSystem;
use generator::{Generator, GeneratorActions};
use phony::Phony;
use task::Task;
use utils::{
    check_make_exists,
    logger::{Logger, LoggerActions},
};

mod constants;
mod filesystem;
mod generator;
mod interactive;
mod phony;
mod task;
mod utils;

fn main() {
    let logger = Logger::new();
    let mut tasks = Task::new();
    let mut phony = Phony::new();
    let mut fs = FileSystem::new();
    let generator = Generator::new();

    // Greeting the user
    interactive::welcome_log();

    // Checking if Makefile already exists in current directory
    let exists = check_make_exists();
    if exists {
        return logger.error(&ERROR_MAKEFILE_EXISTS);
    }

    // Getting base path
    interactive::path_prompt(&mut fs);
    // Adding tasks
    interactive::task_prompt(&mut tasks);
    // Adding tasks to '.PHONY' list
    interactive::phony_prompt(&mut phony, &mut tasks);

    // Generating a Makefile
    generator.generate(&mut tasks, &mut phony, &mut fs);
}
