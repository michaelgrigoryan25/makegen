use crate::{
    constants::{
        ERROR_NO_SUCH_TASK_IN_TASK_LIST, ERROR_PHONY_TASK_EXISTS, ERROR_TASK_CANNOT_BE_EMPTY,
        MESSAGE_ADD_TASKS_TO_PHONY, PROMPT_ADD_MORE_PHONY_TASKS, PROMPT_ADD_PHONY_TASKS, PROMPT_ENTER_TASK_NAME,
    },
    interactive::response_as_bool,
    phony::{Phony, PhonyActions},
    task::{Task, TaskActions},
    utils,
};
use colored::Colorize;

pub fn phony_prompt(phony: &mut Phony, tasks: &mut Task) {
    println!("{}", &MESSAGE_ADD_TASKS_TO_PHONY.bold());

    loop {
        // Adding tasks to '.PHONY' list is not necessary
        println!("{}", &PROMPT_ADD_PHONY_TASKS.blue().bold());
        let add_tasks_raw = utils::get_input();
        let add_tasks = response_as_bool(add_tasks_raw);

        // If the user wants to add tasks to the '.PHONY' list
        if add_tasks {
            loop {
                println!("{}", &PROMPT_ENTER_TASK_NAME.blue().bold());

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

                println!("{}", &PROMPT_ADD_MORE_PHONY_TASKS.blue().bold());

                let should_continue_raw = utils::get_input();
                let should_continue = response_as_bool(should_continue_raw);
                if !should_continue {
                    break;
                }
            }
        }

        break;
    }
}
