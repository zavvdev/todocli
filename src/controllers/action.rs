use crate::{
    config::{ProcessError, ProcessResult, C_ADD, C_CLEAR, C_EDIT, C_REMOVE},
    models::{
        list::List,
        state::{State, Status},
    },
    parsers::command_parser::{self, ParseResult},
    validators,
};

fn invalid_arguments_result() -> ProcessResult {
    println!("Invalid arguments");
    ProcessResult::Ok
}

fn extract_index_from_args(args: &Vec<&str>) -> usize {
    args.first().unwrap().parse::<usize>().unwrap() - 1
}

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

pub fn edit(parse_result: ParseResult, list: &mut List, state: &mut State) -> ProcessResult {
    if validators::is_arguments_integer(&parse_result.arguments) {
        let index = self::extract_index_from_args(&parse_result.arguments);

        return match list.get(index) {
            Some(task) => {
                println!("Provide new text for task: {}", task.text);
                state.set(C_EDIT, Status::NeedPlainText, Some(index));
                ProcessResult::Ok
            }
            None => ProcessResult::Error(ProcessError::ListItemNotFound),
        };
    }

    self::invalid_arguments_result()
}

pub fn edit_text(raw_input: String, list: &mut List, state: &mut State) -> ProcessResult {
    if let Some(index) = state.task_index {
        match list.alter(index, raw_input) {
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

pub fn remove(parse_result: ParseResult, list: &mut List, state: &mut State) -> ProcessResult {
    if validators::is_arguments_integer(&parse_result.arguments) {
        let index = self::extract_index_from_args(&parse_result.arguments);

        return match list.get(index) {
            Some(task) => {
                println!(
                    "Are you sure you want to remove \"{}\" task? (Yes/No)",
                    task.text
                );
                state.set(C_REMOVE, Status::NeedConfirmation, Some(index));
                ProcessResult::Ok
            }
            None => ProcessResult::Error(ProcessError::ListItemNotFound),
        };
    }

    self::invalid_arguments_result()
}

pub fn remove_confirm(raw_input: String, list: &mut List, state: &mut State) -> ProcessResult {
    if command_parser::is_confirm(&raw_input) {
        if let Some(index) = state.task_index {
            match list.remove(index) {
                Ok(()) => {
                    println!("Task removed successfully");
                    state.reset();
                    ProcessResult::Ok
                }
                Err(cause) => ProcessResult::Error(cause),
            }
        } else {
            ProcessResult::Error(ProcessError::TaskIndexMissing)
        }
    } else {
        ProcessResult::Ok
    }
}

// ==========================================================

pub fn done(parse_result: ParseResult, list: &mut List) -> ProcessResult {
    if validators::is_arguments_integer(&parse_result.arguments) {
        let index = self::extract_index_from_args(&parse_result.arguments);

        return match list.mark_done(index) {
            Ok(()) => {
                println!("Task marked as DONE");
                ProcessResult::Ok
            }
            Err(cause) => ProcessResult::Error(cause),
        };
    }

    self::invalid_arguments_result()
}

// ==========================================================

pub fn undone(parse_result: ParseResult, list: &mut List) -> ProcessResult {
    if validators::is_arguments_integer(&parse_result.arguments) {
        let index = self::extract_index_from_args(&parse_result.arguments);

        return match list.mark_undone(index) {
            Ok(()) => {
                println!("Task marked as UNDONE");
                ProcessResult::Ok
            }
            Err(cause) => ProcessResult::Error(cause),
        };
    }

    self::invalid_arguments_result()
}

// ==========================================================

pub fn clear(state: &mut State) -> ProcessResult {
    println!("Are you sure you want to remove all tasks? (Yes/No)");
    state.set(C_CLEAR, Status::NeedConfirmation, None);
    ProcessResult::Ok
}

pub fn clear_confirm(raw_input: String, list: &mut List, state: &mut State) -> ProcessResult {
    if command_parser::is_confirm(&raw_input) {
        list.clear();
        println!("All tasks removed sucessfully");
    }
    
    state.reset();
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
