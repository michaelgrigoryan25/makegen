use crate::{
    filesystem::{FileSystem, FileSystemActions},
    phony::{Phony, PhonyActions},
    task::{Task, TaskActions},
};

#[derive(Debug, Clone, Default)]
pub struct Generator;

pub trait GeneratorActions {
    fn generate(&self, tasks: &mut Task, phony: &mut Phony, fs: &mut FileSystem);
}

impl Generator {
    pub fn new() -> Generator {
        Generator {}
    }
}

impl GeneratorActions for Generator {
    fn generate(&self, tasks: &mut Task, phony: &mut Phony, fs: &mut FileSystem) {
        let mut data = String::new();

        if !phony.consume_phony_list().is_empty() {
            let phony_list_string = phony.get_phony_list_string();
            data += &format!("\n{}", &phony_list_string).to_string();
            println!("{}", phony_list_string);
        }

        if !tasks.consume_task_list().is_empty() {
            let task_list_string = tasks.get_task_list_string();
            data += &format!("\n{}", task_list_string).to_string();
        }

        fs.write_buffer(&mut data)
    }
}
