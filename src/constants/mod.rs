pub const VERSION: &str = "0.0.1";
pub const PROMPT_ADD_TASKS: &str = "Add tasks";
pub const SELECTED_PATH_MESSAGE: &str = "Selected path:";
pub const DEFAULTING_TO_MESSAGE: &str = "Defaulting to:";
pub const WELCOME_MESSAGE: &str = "Makefile generator v";
pub const ADD_TASKS_TO_PHONY_MESSAGE: &str = "Add tasks to '.PHONY' list";
pub const DEFAULT_OUT_DIR_MESSAGE: &str = "Output directory(Default: './'):";

pub const MAKEFILE_FILENAME: &str = "Makefile";
pub const PROMPT_ENTER_TASK_NAME: &str = "Enter task name: ";
pub const PROMPT_ENTER_TASK_COMMAND: &str = "Enter the command that you want to execute with this task։ ";

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

pub const FINISH_ADDING_TASKS_MESSAGE: &str = "Do you want to finish adding tasks? (Y/n)";
pub const ADD_MORE_PHONY_TASKS_PROMPT: &str = "Do you want to add more tasks to '.PHONY' list? (Y/n)";

pub const GENERATED_BY_MAKEGEN_COMMENT: &str = "# This file was generated using the `makegen` cli tool";
