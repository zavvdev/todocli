use crate::{
    config::{ProcessError, ProcessResult, C_ADD, C_EDIT},
    models::{
        list::List,
        state::{State, Status},
    },
};

// ==========================================================

pub fn exit() -> ProcessResult {
    println!("Bye!");
    ProcessResult::Terminate
}

// ==========================================================

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

// ==========================================================

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

// ==========================================================

pub fn add(state: &mut State) -> ProcessResult {
    println!("Enter your todo");
    state.set(C_ADD, Status::NeedPlainText, None);
    ProcessResult::Ok
}

pub fn add_text(text: String, list: &mut List, state: &mut State) -> ProcessResult {
    match list.add(text) {
        Ok(()) => {
            println!("Task added successfully");
            state.reset();
            ProcessResult::Ok
        }
        Err(cause) => ProcessResult::Error(cause),
    }
}

// ==========================================================

pub fn edit(task_index: usize, list: &mut List, state: &mut State) -> ProcessResult {
    match list.get(task_index) {
        Some(task) => {
            println!("Provide new text for task: {}", task.text);
            state.set(C_EDIT, Status::NeedPlainText, Some(task_index));
            ProcessResult::Ok
        }
        None => ProcessResult::Error(ProcessError::ListItemNotFound),
    }
}

pub fn edit_text(text: String, list: &mut List, state: &mut State) -> ProcessResult {
    if let Some(index) = state.task_index {
        match list.alter(index, text) {
            Ok(()) => {
                println!("Task edited successfully");
                state.reset();
                ProcessResult::Ok
            }
            Err(cause) => ProcessResult::Error(cause),
        }
    } else {
        ProcessResult::Error(ProcessError::TaskIndexMissing)
    }
}

// ==========================================================

pub fn remove(task_index: usize) -> ProcessResult {
    println!("{}", task_index);

    ProcessResult::Ok
}

// ==========================================================

pub fn done(task_index: usize) -> ProcessResult {
    println!("{}", task_index);

    ProcessResult::Ok
}

// ==========================================================

pub fn undone(task_index: usize) -> ProcessResult {
    println!("{}", task_index);

    ProcessResult::Ok
}

// ==========================================================

pub fn save() -> ProcessResult {
    ProcessResult::Ok
}

// ==========================================================

pub fn load() -> ProcessResult {
    ProcessResult::Ok
}

// ==========================================================

pub fn clear() -> ProcessResult {
    ProcessResult::Ok
}

// ==========================================================
