use std::{
    fs::File,
    io::{self, Write},
};

use crate::{
    config::{ProcessError, ProcessResult, C_ADD, C_CLEAR, C_EDIT, C_REMOVE, C_SAVE},
    models::{
        list::List,
        state::{State, Status},
    },
    parsers::{
        command_parser::{self, ParseResult},
        task_parser,
    },
    validators,
};

fn extract_index_from_args(args: &Vec<&str>) -> usize {
    args.first().unwrap().parse::<usize>().unwrap() - 1
}

// ==========================================================

pub fn exit() -> ProcessResult {
    ProcessResult::Terminate
}

// ==========================================================

pub fn help() -> ProcessResult {
    let feedback = "exit     - Exit program
help     - View available commands
list     - View all tasks
clear    - Clear tasks
add      - Add new task
edit 2   - Edit task by index where 2 is index
remove 2 - Delete task by index where 2 is index
done 2   - Mark task as DONE where 2 is index
undone 2 - Mark task as UNDONE where 2 is index
save     - Save list to external file
load     - Load list from external file";

    ProcessResult::Feedback(feedback.to_string())
}

// ==========================================================

pub fn list(l: &mut List) -> ProcessResult {
    ProcessResult::Feedback(task_parser::to_text(&l.dump()))
}

// ==========================================================

pub fn add(state: &mut State) -> ProcessResult {
    state.set(C_ADD, Status::NeedPlainText, None);
    ProcessResult::Feedback("enter task".to_string())
}

pub fn add_text(text: String, list: &mut List, state: &mut State) -> ProcessResult {
    match list.add(text) {
        Ok(()) => {
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
                state.set(C_EDIT, Status::NeedPlainText, Some(index));
                ProcessResult::Feedback(format!("provide new text for task: {}", task.text))
            }
            None => ProcessResult::Error(ProcessError::ListItemNotFound),
        };
    }

    ProcessResult::Error(ProcessError::InvalidArguments)
}

pub fn edit_text(raw_input: String, list: &mut List, state: &mut State) -> ProcessResult {
    if let Some(index) = state.task_index {
        match list.alter(index, raw_input) {
            Ok(()) => {
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
                state.set(C_REMOVE, Status::NeedConfirmation, Some(index));
                ProcessResult::Feedback(format!("remove \"{}\" task? (yes/no)", task.text))
            }
            None => ProcessResult::Error(ProcessError::ListItemNotFound),
        };
    }

    ProcessResult::Error(ProcessError::InvalidArguments)
}

pub fn remove_confirm(raw_input: String, list: &mut List, state: &mut State) -> ProcessResult {
    if command_parser::is_confirm(&raw_input) {
        if let Some(index) = state.task_index {
            match list.remove(index) {
                Ok(()) => {
                    state.reset();
                    ProcessResult::Ok
                }
                Err(cause) => ProcessResult::Error(cause),
            }
        } else {
            ProcessResult::Error(ProcessError::TaskIndexMissing)
        }
    } else {
        ProcessResult::Sh
    }
}

// ==========================================================

pub fn done(parse_result: ParseResult, list: &mut List) -> ProcessResult {
    if validators::is_arguments_integer(&parse_result.arguments) {
        let index = self::extract_index_from_args(&parse_result.arguments);

        return match list.mark_done(index) {
            Ok(()) => ProcessResult::Ok,
            Err(cause) => ProcessResult::Error(cause),
        };
    }

    ProcessResult::Error(ProcessError::InvalidArguments)
}

// ==========================================================

pub fn undone(parse_result: ParseResult, list: &mut List) -> ProcessResult {
    if validators::is_arguments_integer(&parse_result.arguments) {
        let index = self::extract_index_from_args(&parse_result.arguments);

        return match list.mark_undone(index) {
            Ok(()) => ProcessResult::Ok,
            Err(cause) => ProcessResult::Error(cause),
        };
    }

    ProcessResult::Error(ProcessError::InvalidArguments)
}

// ==========================================================

pub fn clear(list: &mut List, state: &mut State) -> ProcessResult {
    if list.is_empty() {
        ProcessResult::Feedback("empty".to_string())
    } else {
        println!();
        state.set(C_CLEAR, Status::NeedConfirmation, None);
        ProcessResult::Feedback("remove all? (yes/no)".to_string())
    }
}

pub fn clear_confirm(raw_input: String, list: &mut List, state: &mut State) -> ProcessResult {
    state.reset();

    if command_parser::is_confirm(&raw_input) {
        list.clear();
        ProcessResult::Ok
    } else {
        ProcessResult::Sh
    }
}

// ==========================================================

pub fn save(list: &mut List, state: &mut State) -> ProcessResult {
    if list.is_empty() {
        ProcessResult::Feedback("empty".to_string())
    } else {
        state.set(C_SAVE, Status::NeedPlainText, None);
        ProcessResult::Feedback("where?".to_string())
    }
}

pub fn save_text(raw_input: String, list: &mut List, state: &mut State) -> ProcessResult {
    let mut result = ProcessResult::Ok;

    match File::create(raw_input) {
        io::Result::Ok(ref mut file) => {
            match file.write_all(task_parser::to_text(&list.dump()).as_bytes()) {
                Ok(..) => {
                    state.reset();
                }
                Err(..) => {
                    result = ProcessResult::Error(ProcessError::CannotWriteToFile);
                }
            }
        }
        io::Result::Err(..) => {
            result = ProcessResult::Error(ProcessError::CannotCreateFile);
        }
    };

    result
}

// ==========================================================

pub fn load() -> ProcessResult {
    ProcessResult::Ok
}

// ==========================================================