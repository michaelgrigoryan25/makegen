use crate::constants::{
    ERROR_ACCESS_DENIED, ERROR_CONVERTING_PATHBUF, ERROR_READING_LINE, MAKEFILE_FILENAME,
};
use std::{env, fs, io};

pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect(ERROR_READING_LINE);
    input.trim().to_string()
}

pub fn dir_path_as_string() -> String {
    env::current_dir()
        .expect(ERROR_ACCESS_DENIED)
        .to_str()
        .expect(ERROR_CONVERTING_PATHBUF)
        .to_string()
}

pub fn check_make_exists() -> bool {
    let path = dir_path_as_string();
    let dir = fs::read_dir(path).expect(ERROR_ACCESS_DENIED).flatten();

    for i in dir {
        if i.file_name() == MAKEFILE_FILENAME {
            return true;
        }
    }

    false
}
