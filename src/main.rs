use std::io::{self, Write};

struct Task {
    id: u64,
    text: String,
    is_done: bool,
}

impl Task {
    fn new(id: u64, text: String) -> Self {
        Self {
            id,
            text,
            is_done: false,
        }
    }

    fn done(&mut self) {
        self.is_done = true;
    }

    fn undone(&mut self) {
        self.is_done = false;
    }
}

struct List {
    tasks: Vec<Task>,
}

impl List {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add(&mut self, task: String) {
        match self.tasks.last() {
            Some(last) => {
                let next_id = last.id + 1; // TODO: Check for integer overflow
                self.tasks.push(Task::new(next_id, task));
            }
            None => {
                self.tasks.push(Task::new(1, task));
            }
        }
    }

    fn remove(&mut self, id: u64) {
        self.tasks.retain(|t| t.id != id);
    }

    fn clear(&mut self) {
        self.tasks.clear();
    }

    fn mark_task_done(&mut self, id: u64) {
        for task in &mut self.tasks {
            if task.id == id && !task.is_done {
                task.done();
                break;
            }
        }
    }

    fn mark_task_undone(&mut self, id: u64) {
        for task in &mut self.tasks {
            if task.id == id && task.is_done {
                task.undone();
                break;
            }
        }
    }

    fn view(&mut self) {
        for task in &mut self.tasks {
            println!("{} | {}", task.id, task.text);
        }
    }
}

enum ProcessResult {
    Exit,
    UnknownCommand,
    Ok,
}

const EXIT: &str = "exit";
const HELP: &str = "help";

fn trim_whitespaces(s: &str) -> String {
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

fn exit() {
    println!("Bye!");
}

fn help() {
    println!("Available commands:");
    println!("exit                - terminate the program");
    println!("help                - display help message");
    println!("list                - print all tasks");
    println!("add [task]          - add new task");
    println!("remove [...task_id] - remove items or options");
    println!("done [...task_id]   - mark items or options as done");
    println!("undone [...task_id] - mark items as undone");
    println!("clear               - remove all tasks");
    println!("save                - save the current list to file");
    println!("load                - load a list from a file");
}

fn process_input(input: &str, list: &mut List) -> ProcessResult {
    if input == EXIT {
        exit();
        return ProcessResult::Exit;
    }

    if input == HELP {
        help();
        return ProcessResult::Ok;
    }

    if input == "add" {
        list.add("hello".to_string());
        return ProcessResult::Ok;
    }

    if input == "list" {
        list.view();
        return ProcessResult::Ok;
    }

    ProcessResult::UnknownCommand
}

fn prepare_input(s: &str) -> String {
    trim_whitespaces(s)
}

fn accept_input() -> String {
    let mut input = String::new();
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

fn main() {
    let mut list = List::new();

    loop {
        print!("> ");

        match process_input(&prepare_input(&accept_input()), &mut list) {
            ProcessResult::Exit => break,
            ProcessResult::UnknownCommand => {
                println!("Unknown command");
            }
            ProcessResult::Ok => {}
        }
    }
}
