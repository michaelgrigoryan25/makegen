use fs::FS;
use phony::Phony;
use task::Task;

mod constants;
mod fs;
mod interactive;
mod phony;
mod task;
mod utils;

fn main() {
    let mut fs = FS::new();
    let mut tasks = Task::new();
    let mut phony = Phony::new();

    // Greeting the user
    interactive::welcome_log();
    // Getting base path
    interactive::path_prompt(&mut fs);
    // Adding tasks
    interactive::task_prompt(&mut tasks);
    // Adding tasks to '.PHONY' list
    interactive::phony_prompt(&mut phony, &mut tasks)
}
