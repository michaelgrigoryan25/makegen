use crate::utils;

mod path_prompt;
mod task_prompt;
mod welcome_log;
mod phony_prompt;

// Exporting the functions
pub use path_prompt::path_prompt;
pub use task_prompt::task_prompt;
pub use welcome_log::welcome_log;
pub use phony_prompt::phony_prompt;

// For checking if the user wants to finish a task
// TODO: Fix the bug with `N` values not returning false
fn response_as_bool() -> bool {
    let response = utils::get_input().trim().to_lowercase();
    let finish = match response.as_ref() {
        "y" => true,
        "n" => false,
        _ => true,
    };

    finish
}
