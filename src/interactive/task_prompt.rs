use crate::{
    constants::{
        ERROR_COMMAND_CANNOT_BE_EMPTY, ERROR_TASK_CANNOT_BE_EMPTY, PROMPT_ADD_TASKS, PROMPT_CONTINUE_ADDING_TASKS,
        PROMPT_ENTER_TASK_COMMAND, PROMPT_ENTER_TASK_NAME,
    },
    interactive::response_as_bool,
    task::{Task, TaskActions},
    utils::{
        self,
        logger::{Logger, LoggerActions},
    },
};

// Prompt for adding tasks
pub fn task_prompt(tasks: &mut Task) {
    // #[allow(unused_variables, unused_mut)]
    // let mut is_first_task = true;
    let logger = Logger::new();

    logger.log(&PROMPT_ADD_TASKS);

    loop {
        // Getting task name
        logger.info(&PROMPT_ENTER_TASK_NAME);

        let task_name = utils::get_input();
        if task_name.is_empty() {
            logger.error(&ERROR_TASK_CANNOT_BE_EMPTY);
            continue;
        }

        // Getting task command
        logger.info(&PROMPT_ENTER_TASK_COMMAND);
        let command = utils::get_input();
        if command.is_empty() {
            logger.error(&ERROR_COMMAND_CANNOT_BE_EMPTY);
            continue;
        }

        // Adding the task to the list
        tasks.add_task(task_name, command);

        logger.info(&PROMPT_CONTINUE_ADDING_TASKS);

        let yes_raw = utils::get_input();
        let yes = response_as_bool(yes_raw.to_owned());

        if !yes {
            break;
        };
    }
}
