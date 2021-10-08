use crate::{
    constants::{
        ERROR_NO_SUCH_TASK_IN_TASK_LIST, ERROR_PHONY_TASK_EXISTS, ERROR_TASK_CANNOT_BE_EMPTY,
        MESSAGE_ADD_TASKS_TO_PHONY, PROMPT_ADD_MORE_PHONY_TASKS, PROMPT_ADD_PHONY_TASKS, PROMPT_ENTER_TASK_NAME,
    },
    interactive::response_as_bool,
    phony::{Phony, PhonyActions},
    task::{Task, TaskActions},
    utils::{
        self,
        logger::{Logger, LoggerActions},
    },
};

pub fn phony_prompt(phony: &mut Phony, tasks: &mut Task) {
    let logger = Logger::new();
    logger.log(&MESSAGE_ADD_TASKS_TO_PHONY);

    loop {
        // Adding tasks to '.PHONY' list is not necessary
        logger.warn(&PROMPT_ADD_PHONY_TASKS);

        let add_tasks_raw = utils::get_input();
        let add_tasks = response_as_bool(add_tasks_raw);

        // If the user wants to add tasks to the '.PHONY' list
        if add_tasks {
            loop {
                logger.log(&PROMPT_ENTER_TASK_NAME);
                let task_name = utils::get_input();

                // If task name is empty
                if task_name.is_empty() {
                    logger.error(&ERROR_TASK_CANNOT_BE_EMPTY);
                    continue;
                } else {
                    // If there is no such task with the specified name
                    if !tasks.contains_task_with_name(task_name.to_owned()) {
                        logger.error(&ERROR_NO_SUCH_TASK_IN_TASK_LIST);
                        continue;
                    }

                    // If phony task has already been added to the list
                    if phony.consume_phony_list().contains(&task_name) {
                        logger.error(&ERROR_PHONY_TASK_EXISTS);
                        continue;
                    }
                }

                // Adding the task to `.PHONY` list
                phony.add_phony(task_name);

                logger.warn(&PROMPT_ADD_MORE_PHONY_TASKS);
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
