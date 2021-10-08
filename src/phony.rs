use crate::constants::PREFIX_PHONY_LIST;

#[derive(Debug, Clone, Default)]
pub struct Phony {
    tasks: Vec<String>,
}

pub trait PhonyActions {
    fn get_phony_list_string(&mut self) -> String;
    fn consume_phony_list(&mut self) -> Vec<String>;
    fn add_phony(&mut self, task: String) -> Vec<String>;
    fn del_phony(&mut self, task: String) -> Vec<String>;
}

impl Phony {
    pub fn new() -> Phony {
        Phony { tasks: vec![] }
    }
}

impl PhonyActions for Phony {
    // For getting `.PHONY` list as a string
    fn get_phony_list_string(&mut self) -> String {
        // Prefixing the file with auto generated comment
        let mut phony_list_string = String::from(PREFIX_PHONY_LIST);

        for task in &self.tasks {
            phony_list_string += task;
        }

        phony_list_string
    }

    // For using the vector of tasks
    fn consume_phony_list(&mut self) -> Vec<String> {
        self.tasks.to_vec()
    }

    // For adding a task to `.PHONY` list
    fn add_phony(&mut self, task: String) -> Vec<String> {
        self.tasks.push(task);
        self.tasks.to_owned()
    }

    // For removing a task from `.PHONY` list
    fn del_phony(&mut self, task: String) -> Vec<String> {
        self.tasks
            .to_owned()
            .into_iter()
            .filter(|t| task != *t)
            .collect::<Vec<String>>()
    }
}
