use crate::{
    constants::{DEFAULTING_TO_MESSAGE, DEFAULT_OUT_DIR_MESSAGE, SELECTED_PATH_MESSAGE},
    fs::{FsActions, FS},
    utils,
};
use colored::Colorize;

// Prompt for getting the desired path from the user
pub fn path_prompt(fs: &mut FS) {
    println!("{}", &DEFAULT_OUT_DIR_MESSAGE.blue());

    // Getting and setting the base directory
    let base_path = utils::get_input();

    // Defaulting to base path provided by `fs`
    if !base_path.is_empty() {
        fs.set_base_path(base_path.to_owned());

        println!(
            "{msg} '{path}'",
            msg = &SELECTED_PATH_MESSAGE,
            path = base_path
        );
    } else {
        println!(
            "{msg} '{path}'",
            msg = &DEFAULTING_TO_MESSAGE,
            path = fs.get_base_path()
        );
    }
}
