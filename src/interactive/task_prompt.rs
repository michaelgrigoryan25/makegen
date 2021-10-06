use crate::{
    constants::{
        ERROR_COMMAND_CANNOT_BE_EMPTY, ERROR_TASK_CANNOT_BE_EMPTY, PROMPT_ADD_TASKS, PROMPT_CONTINUE_ADDING_TASKS,
        PROMPT_ENTER_TASK_COMMAND, PROMPT_ENTER_TASK_NAME,
    },
    interactive::response_as_bool,
    task::{Task, TaskActions},
    utils,
};
use colored::Colorize;

// Prompt for adding tasks
// TODO: Add dependency handling
pub fn task_prompt(tasks: &mut Task) {
    #[allow(unused_variables, unused_mut)]
    let mut is_first_task = true;

    println!("{}", &PROMPT_ADD_TASKS.bold());

    loop {
        // Getting task name
        println!("{}", &PROMPT_ENTER_TASK_NAME.blue().bold());

        let task_name = utils::get_input();
        if task_name.is_empty() {
            println!("{}", &ERROR_TASK_CANNOT_BE_EMPTY.red().bold());
            continue;
        }

        // Getting task command
        println!("{}", &PROMPT_ENTER_TASK_COMMAND.blue().bold());
        let command = utils::get_input();
        if command.is_empty() {
            println!("{}", &ERROR_COMMAND_CANNOT_BE_EMPTY.red().bold());
            continue;
        }

        // Adding the task to the list
        tasks.add_task(task_name, command);

        println!("{}", &PROMPT_CONTINUE_ADDING_TASKS.blue().bold());

        let yes_raw = utils::get_input();
        let yes = response_as_bool(yes_raw.to_owned());

        if !yes {
            break;
        };
    }
}
