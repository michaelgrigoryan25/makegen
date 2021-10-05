#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Command {
    name: String,
    exec: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Task {
    commands: Vec<Command>,
}

pub trait TaskActions {
    fn consume_tasks(&mut self) -> Vec<Command>;
    fn del_task(&mut self, name: String) -> Vec<Command>;
    fn contains_task_with_name(&mut self, name: String) -> bool;
    fn add_task(&mut self, name: String, exec: String) -> Vec<Command>;
}

impl Task {
    pub fn new() -> Task {
        Task { commands: vec![] }
    }
}

impl Command {
    pub fn new(name: String, exec: String) -> Command {
        Command { name, exec }
    }
}

impl TaskActions for Task {
    fn consume_tasks(&mut self) -> Vec<Command> {
        self.commands.to_vec()
    }

    #[allow(unused_must_use)]
    fn del_task(&mut self, name: String) -> Vec<Command> {
        self.commands.iter().filter(|&c| c.name != *name);
        self.commands.to_vec()
    }

    fn add_task(&mut self, name: String, exec: String) -> Vec<Command> {
        let command = Command::new(name, exec);
        self.commands.push(command);
        self.commands.to_vec()
    }

    fn contains_task_with_name(&mut self, name: String) -> bool {
        for command in &self.commands {
            if command.name == name {
                return true;
            }
        }

        false
    }
}
