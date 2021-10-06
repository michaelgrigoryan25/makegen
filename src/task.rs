#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Command {
    name: String,
    exec: String,
    depends_on: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Task {
    commands: Vec<Command>,
}

pub trait TaskActions {
    fn consume_task_list(&mut self) -> Vec<Command>;
    fn get_task_list_string(&mut self) -> String;
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
        Command {
            name,
            exec,
            depends_on: None,
        }
    }
}

impl TaskActions for Task {
    fn consume_task_list(&mut self) -> Vec<Command> {
        self.commands.to_vec()
    }

    #[allow(unused_must_use)]
    fn del_task(&mut self, name: String) -> Vec<Command> {
        self.commands.iter().filter(|&c| c.name != *name);
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

    fn add_task(&mut self, name: String, exec: String) -> Vec<Command> {
        let command = Command::new(name, exec);
        self.commands.push(command);
        self.commands.to_vec()
    }

    fn get_task_list_string(&mut self) -> String {
        let mut task_list_string = String::new();

        for task in self.consume_task_list() {
            task_list_string += format!("\n\n{task}:\n", task = &task.name).as_str();
            task_list_string += format!("\t{cmd}", cmd = &task.exec).as_str();
        }

        task_list_string
    }
}
