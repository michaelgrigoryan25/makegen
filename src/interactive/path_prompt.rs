use crate::{
    constants::{MESSAGE_DEFAULTING_TO_PATH, MESSAGE_OUTPUT_DIR, MESSAGE_SELECTED_PATH},
    filesystem::{FileSystem, FileSystemActions},
    utils::{
        self, dir_path_as_string, get_file_path_from_current_dir,
        logger::{Logger, LoggerActions},
    },
};

// Prompt for getting the desired path from the user
pub fn path_prompt(fs: &mut FileSystem) {
    let logger = Logger::new();
    let current_dir = dir_path_as_string();

    logger.warn(MESSAGE_OUTPUT_DIR);

    // Getting and setting the base directory
    let base_path = utils::get_input();

    // Defaulting to base path provided by `fs`
    if !base_path.is_empty() {
        let base_path_formatted = get_file_path_from_current_dir(base_path);
        let message = format!(
            "{msg} '{path}'",
            msg = &MESSAGE_SELECTED_PATH,
            path = base_path_formatted
        );

        // Setting output path
        fs.set_base_path(base_path_formatted);
        logger.success(&message);
    } else {
        let message = format!("{msg} '{path}'", msg = &MESSAGE_DEFAULTING_TO_PATH, path = &current_dir);
        logger.success(&message);
    }
}
