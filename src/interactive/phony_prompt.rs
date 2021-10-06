use crate::{
    constants::{
        ADD_MORE_PHONY_TASKS_PROMPT, ADD_TASKS_TO_PHONY_MESSAGE, ERROR_NO_SUCH_TASK_IN_TASK_LIST,
        ERROR_PHONY_TASK_EXISTS, ERROR_TASK_CANNOT_BE_EMPTY, PROMPT_ENTER_TASK_NAME,
    },
    interactive::response_as_bool,
    phony::{Phony, PhonyActions},
    task::{Task, TaskActions},
    utils,
};
use colored::Colorize;

pub fn phony_prompt(phony: &mut Phony, tasks: &mut Task) {
    println!("{}", &ADD_TASKS_TO_PHONY_MESSAGE.bold());

    loop {
        println!("{}", &PROMPT_ENTER_TASK_NAME.blue());

        let task_name = utils::get_input();

        // If task name is empty
        if task_name.is_empty() {
            println!("{}", &ERROR_TASK_CANNOT_BE_EMPTY.red().bold());
            continue;
        } else {
            // If there is no such task with the specified name
            if !tasks.contains_task_with_name(task_name.to_owned()) {
                println!("{}", &ERROR_NO_SUCH_TASK_IN_TASK_LIST.red().bold());
                continue;
            }

            // If phony task has already been added to the list
            if phony.consume_phony_list().contains(&task_name) {
                println!("{}", &ERROR_PHONY_TASK_EXISTS.red().bold())
            }
        }

        println!("{}", &ADD_MORE_PHONY_TASKS_PROMPT.blue());

        let finish_response = utils::get_input();
        let finished = response_as_bool(finish_response);
        if finished {
            break;
        }
    }
}
