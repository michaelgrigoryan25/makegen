use crate::constants::ERROR_READING_LINE;
use std::io;

pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect(ERROR_READING_LINE);
    input.trim().to_string()
}
