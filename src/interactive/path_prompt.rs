use crate::{
    constants::{MESSAGE_DEFAULTING_TO_PATH, MESSAGE_DEFAULT_OUT_DIR, MESSAGE_SELECTED_PATH},
    filesystem::{FileSystem, FileSystemActions},
    utils,
};
use colored::Colorize;

// Prompt for getting the desired path from the user
pub fn path_prompt(fs: &mut FileSystem) {
    println!("{}", &MESSAGE_DEFAULT_OUT_DIR.blue());

    // Getting and setting the base directory
    let base_path = utils::get_input();

    // Defaulting to base path provided by `fs`
    if !base_path.is_empty() {
        fs.set_base_path(base_path.to_owned());
        println!("{msg} '{path}'", msg = &MESSAGE_SELECTED_PATH, path = base_path);
    } else {
        println!(
            "{msg} '{path}'",
            msg = &MESSAGE_DEFAULTING_TO_PATH,
            path = fs.get_base_path()
        );
    }
}
