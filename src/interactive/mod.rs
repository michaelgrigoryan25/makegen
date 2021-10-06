mod path_prompt;
mod phony_prompt;
mod task_prompt;
mod welcome_log;

// Exporting the functions
pub use path_prompt::path_prompt;
pub use phony_prompt::phony_prompt;
pub use task_prompt::task_prompt;
pub use welcome_log::welcome_log;

// For checking if the user wants to finish a task
// TODO: Fix the bug with `N` values not returning false
fn response_as_bool(response: String) -> bool {
    match response.to_lowercase().as_ref() {
        "y" => true,
        "n" => false,
        _ => true,
    }
}

#[test]
fn test_response_as_bool() {
    assert_eq!(true, response_as_bool("y".to_string()));
    assert_eq!(false, response_as_bool("n".to_string()));
    assert_eq!(true, response_as_bool("Y".to_string()));
    assert_eq!(false, response_as_bool("N".to_string()));
    assert_eq!(true, response_as_bool("something".to_string()));
}
