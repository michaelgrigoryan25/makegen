use crate::constants::{ERROR_ACCESS_DENIED, ERROR_CONVERTING_PATHBUF, ERROR_READING_LINE, FILE_MAKEFILE_DEFAULT_NAME};
use std::{env, fs::OpenOptions, io, path::PathBuf};

// For getting clean, trimmed input from the terminal
pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect(ERROR_READING_LINE);
    input.trim().to_string()
}

// For getting current path as string
pub fn dir_path_as_string() -> String {
    env::current_dir()
        .expect(ERROR_ACCESS_DENIED)
        .to_str()
        .expect(ERROR_CONVERTING_PATHBUF)
        .to_string()
}

// For getting a valid path from current workspace
pub fn get_file_path_from_current_dir(path: String) -> String {
    let dir = dir_path_as_string();
    let file_dir = format!("{}/{}", &dir, &path);

    PathBuf::from(&file_dir)
        .to_str()
        .expect(ERROR_CONVERTING_PATHBUF)
        .to_string()
}

// For checking if there is an existing makefile in current workspace
pub fn check_make_exists() -> bool {
    let path = dir_path_as_string();
    // Getting the Makefile in current workspace
    let makefile_path = format!("{}/{}", &path, &FILE_MAKEFILE_DEFAULT_NAME);
    OpenOptions::new().read(true).open(&makefile_path).is_ok()
}
