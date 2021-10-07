use crate::{
    constants::{MESSAGE_DEFAULTING_TO_PATH, MESSAGE_OUTPUT_DIR, MESSAGE_SELECTED_PATH},
    filesystem::{FileSystem, FileSystemActions},
    utils::{self, dir_path_as_string, get_file_path_from_current_dir},
};
use colored::Colorize;

// Prompt for getting the desired path from the user
pub fn path_prompt(fs: &mut FileSystem) {
    print!("{} ", &MESSAGE_OUTPUT_DIR.blue());
    println!("({} {})", &MESSAGE_DEFAULTING_TO_PATH, dir_path_as_string());

    // Getting and setting the base directory
    let base_path = utils::get_input();

    // Defaulting to base path provided by `fs`
    if !base_path.is_empty() {
        fs.set_base_path(base_path.to_owned());
        let base_path_formatted = get_file_path_from_current_dir(base_path.clone());
        println!("{msg} '{path}'", msg = &MESSAGE_SELECTED_PATH, path = base_path_formatted);
    } else {
        println!(
            "{msg} '{path}'",
            msg = &MESSAGE_DEFAULTING_TO_PATH,
            path = fs.get_base_path()
        );
    }
}
