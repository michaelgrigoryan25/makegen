pub const VERSION: &str = "0.0.1";

pub const MESSAGE_SELECTED_PATH: &str = "Selected path:";
pub const MESSAGE_OUTPUT_DIR: &str = "Output directory:";
pub const MESSAGE_WELCOME_LOG: &str = "Makefile generator v";
pub const MESSAGE_DEFAULTING_TO_PATH: &str = "Defaulting to:";
pub const MESSAGE_ADD_TASKS_TO_PHONY: &str = "Add tasks to '.PHONY' list";
pub const MESSAGE_MAKEFILE_GENERATED: &str = "Makefile has been generated successfully";

pub const FILE_MAKEFILE_DEFAULT_NAME: &str = "Makefile";
pub const FILE_COMMENT_GENERATED_BY_MAKEGEN: &str = "# This file was generated using the `makegen` cli tool";

pub const ERROR_TASK_CANNOT_BE_EMPTY: &str = "Task name can't be empty";
pub const ERROR_COMMAND_CANNOT_BE_EMPTY: &str = "Command can't be empty";
pub const ERROR_READING_LINE: &str = "There was an error reading the input";
pub const ERROR_OPENING_FILE: &str = "There was an error while opening the file";
pub const ERROR_WRITING_FILE: &str = "There was an error while creating the Makefile";
pub const ERROR_NO_SUCH_TASK_IN_TASK_LIST: &str = "There is no such task in the task list";
pub const ERROR_MAKEFILE_EXISTS: &str = "Stopping since a Makefile exists in current workspace";
pub const ERROR_ACCESS_DENIED: &str = "Access denied for writing the file at the specified location";
pub const ERROR_CONVERTING_PATHBUF: &str = "There was an error while converting 'PathBuf' to String";
pub const ERROR_PHONY_TASK_EXISTS: &str = "'.PHONY' task with name {} has already been added to the list";

pub const PROMPT_ADD_TASKS: &str = "Add tasks";
pub const PROMPT_ENTER_TASK_NAME: &str = "Enter task name: ";
pub const PROMPT_ADD_PHONY_TASKS: &str = "Do you want to add tasks to '.PHONY' list (Y/n)";
pub const PROMPT_CONTINUE_ADDING_TASKS: &str = "Do you want to continue adding tasks? (Y/n)";
pub const PROMPT_ADD_MORE_PHONY_TASKS: &str = "Do you want to add more tasks to '.PHONY' list? (Y/n)";
pub const PROMPT_ENTER_TASK_COMMAND: &str = "Enter the command that you want to execute with this task։ ";
