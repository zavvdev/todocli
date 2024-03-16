use crate::{
    config::{ProcessResult, C_ADD},
    models::{
        list::List,
        state::{State, Status},
    },
};

pub fn unknown_command() -> ProcessResult {
    println!("Unknown command");
    ProcessResult::Ok
}

pub fn exit() -> ProcessResult {
    println!("Bye!");
    ProcessResult::Terminate
}

pub fn list(l: &mut List) -> ProcessResult {
    for (index, task) in l.dump().iter().enumerate() {
        let status = match task.is_done {
            true => "[+]",
            false => "[ ]",
        };

        println!("{}) {} {}", index + 1, status, task.text);
    }

    ProcessResult::Ok
}

pub fn add(state: &mut State) -> ProcessResult {
    state.set(C_ADD, Status::NeedMoreData);
    ProcessResult::Ok
}

pub fn help() -> ProcessResult {
    println!("exit     - Exit program");
    println!("help     - View available commands");
    println!("list     - View all tasks");
    println!("clear    - Clear tasks");
    println!("add      - Add new task");
    println!("edit 2   - Edit task by index where 2 is index");
    println!("remove 2 - Delete task by index where 2 is index");
    println!("done 2   - Mark task as DONE where 2 is index");
    println!("undone 2 - Mark task as UNDONE where 2 is index");
    println!("save     - Save list to external file");
    println!("load     - Load list from external file");
    
    ProcessResult::Ok
}

pub fn edit(task_index: usize) -> ProcessResult {
    println!("{}", task_index);

    ProcessResult::Ok
}

pub fn done() -> ProcessResult {
    ProcessResult::Ok
}

pub fn undone() -> ProcessResult {
    ProcessResult::Ok
}

pub fn save() -> ProcessResult {
    ProcessResult::Ok
}

pub fn load() -> ProcessResult {
    ProcessResult::Ok
}

pub fn clear() -> ProcessResult {
    ProcessResult::Ok
}

pub fn remove() -> ProcessResult {
    ProcessResult::Ok
}
