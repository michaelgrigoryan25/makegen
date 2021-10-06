use colored::Colorize;

use crate::{
    constants::{
        ERROR_COMMAND_CANNOT_BE_EMPTY, ERROR_TASK_CANNOT_BE_EMPTY, FINISH_ADDING_TASKS_MESSAGE,
        PROMPT_ADD_TASKS, PROMPT_ENTER_TASK_COMMAND, PROMPT_ENTER_TASK_NAME,
    },
    interactive::response_as_bool,
    task::{Task, TaskActions},
    utils,
};

#[allow(unused_variables)]
// Prompt for adding tasks
pub fn task_prompt(tasks: &mut Task) {
    println!("{}", &PROMPT_ADD_TASKS.bold());

    loop {
        println!("{}", &PROMPT_ENTER_TASK_NAME.blue());

        // Getting task name
        let task_name = utils::get_input();
        if task_name.is_empty() {
            println!("{}", &ERROR_TASK_CANNOT_BE_EMPTY.red().bold());
            continue;
        }

        println!("{}", &PROMPT_ENTER_TASK_COMMAND.blue());

        // Getting task command
        let command = utils::get_input();
        if command.is_empty() {
            println!("{}", &ERROR_COMMAND_CANNOT_BE_EMPTY.red().bold());
            continue;
        }

        // Adding the task to the list
        tasks.add_task(task_name, command);
        println!("{}", &FINISH_ADDING_TASKS_MESSAGE.blue());

        // Checking if the user wants to finish adding tasks
        let finished = response_as_bool();
        // Looping until the user is done with adding tasks
        if finished {
            break;
        }
    }
}
