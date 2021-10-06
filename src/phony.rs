#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
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
    fn add_phony(&mut self, task: String) -> Vec<String> {
        self.tasks.push(task);
        self.tasks.to_owned()
    }

    #[allow(unused_must_use)]
    fn del_phony(&mut self, task: String) -> Vec<String> {
        self.tasks.iter().filter(|&t| &task != t);
        self.tasks.to_owned()
    }

    fn get_phony_list_string(&mut self) -> String {
        let mut phony_list_string = String::from(".PHONY: ");
        self.tasks.iter().for_each(|task| phony_list_string += task);

        phony_list_string
    }

    fn consume_phony_list(&mut self) -> Vec<String> {
        self.tasks.to_vec()
    }
}
