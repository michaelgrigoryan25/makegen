use crate::{
    constants::{
        ERROR_COMMAND_CANNOT_BE_EMPTY, ERROR_TASK_CANNOT_BE_EMPTY, FINISH_ADDING_TASKS_MESSAGE,
        PROMPT_ADD_TASKS, PROMPT_ENTER_TASK_COMMAND, PROMPT_ENTER_TASK_NAME,
    },
    interactive::response_as_bool,
    task::{Task, TaskActions},
    utils,
};
use colored::Colorize;

// Prompt for adding tasks
pub fn task_prompt(tasks: &mut Task) {
    println!("{}", &PROMPT_ADD_TASKS.bold());

    loop {
        // Getting task name
        println!("{}", &PROMPT_ENTER_TASK_NAME.blue());
        let task_name = utils::get_input();
        if task_name.is_empty() {
            println!("{}", &ERROR_TASK_CANNOT_BE_EMPTY.red().bold());
            continue;
        }

        // Getting task command
        println!("{}", &PROMPT_ENTER_TASK_COMMAND.blue());
        let command = utils::get_input();
        if command.is_empty() {
            println!("{}", &ERROR_COMMAND_CANNOT_BE_EMPTY.red().bold());
            continue;
        }

        // Adding the task to the list
        tasks.add_task(task_name, command);

        println!("{}", &FINISH_ADDING_TASKS_MESSAGE.blue());

        let finish_response = utils::get_input();
        let finished = response_as_bool(finish_response);
        if finished {
            break;
        }
    }
}
