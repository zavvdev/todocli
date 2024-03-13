use std::io::{self, Write};

// ==================================

const TASKS_LIST_MAX_CAPACITY: usize = 200;
const EXIT: &str = "exit";
const HELP: &str = "help";

enum ProcessResult {
    Exit,
    UnknownCommand,
    Ok,
}

enum ErrorCause {
    CapacityExceeded,
    NotFound,
}

// ==================================

struct Task {
    text: String,
    is_done: bool,
}

impl Task {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            is_done: false,
        }
    }

    fn done(&mut self) {
        self.is_done = true;
    }

    fn undone(&mut self) {
        self.is_done = false;
    }

    fn alter(&mut self, next_text: &str) {
        self.text = next_text.to_string();
    }
}

// ==================================

struct List {
    tasks: Vec<Task>,
}

impl List {
    fn new() -> Self {
        Self {
            tasks: Vec::with_capacity(TASKS_LIST_MAX_CAPACITY),
        }
    }

    fn add(&mut self, text: &str) -> Result<(), ErrorCause> {
        if self.tasks.len() < self.tasks.capacity() {
            self.tasks.push(Task::new(text));
            return Ok(());
        }

        Err(ErrorCause::CapacityExceeded)
    }

    fn remove(&mut self, index: usize) -> Result<(), ErrorCause> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            return Ok(());
        }

        Err(ErrorCause::NotFound)
    }

    fn alter(&mut self, index: usize, next_text: &str) -> Result<(), ErrorCause> {
        match self.tasks.get(index) {
            Some(task) => {
                task.alter(next_text);
                Ok(())
            }
            None => Err(ErrorCause::NotFound),
        }
    }

    fn clear(&mut self) {
        self.tasks.clear();
    }

    fn mark_done(&mut self, index: usize) -> Result<(), ErrorCause> {
        match self.tasks.get(index) {
            Some(task) => {
                task.done();
                Ok(())
            }
            None => Err(ErrorCause::NotFound),
        }
    }

    fn mark_undone(&mut self, index: usize) -> Result<(), ErrorCause> {
        match self.tasks.get(index) {
            Some(task) => {
                task.undone();
                Ok(())
            }
            None => Err(ErrorCause::NotFound),
        }
    }
}

// ==================================

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
        list.add("hello");
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
