use crate::{
    command_parser::{self, ParseResult},
    config::{
        C_ADD, C_CLEAR, C_DONE, C_EDIT, C_EXIT, C_HELP, C_LIST, C_LOAD, C_REMOVE, C_SAVE, C_UNDONE,
    },
    models::{
        list::{self, List},
        state::{State, Status},
    },
    utils,
};
use std::fs;

pub enum ActionResult {
    Sh,
    Ok,
    Terminate,
    ListFull,
    ListEmpty,
    TaskNotFound,
    FileReadError,
    UnknownCommand,
    InvalidArguments,
    NeedConfirm,
    NeedFilePath,
    NeedTask,
    CannotSave,
    CannotLoad,
    Feedback(String),
}

fn extract_index_from_args(args: &Vec<&str>) -> usize {
    args.first().unwrap().parse::<usize>().unwrap() - 1
}

fn map_list_error(e: list::Error) -> ActionResult {
    match e {
        list::Error::CapacityExceeded => ActionResult::ListFull,
        list::Error::ItemNotFound => ActionResult::TaskNotFound,
        list::Error::InvalidPattern => ActionResult::FileReadError,
    }
}

// =========== Actions ===========

fn exit() -> ActionResult {
    ActionResult::Terminate
}

fn help() -> ActionResult {
    let feedback = "exit     - Exit program
help     - View available commands
list     - View all tasks
clear    - Clear tasks
add      - Add new task
edit 2   - Edit task by index where 2 is index
remove 2 - Delete task by index where 2 is index
done 2   - Mark task as DONE where 2 is index
undone 2 - Mark task as UNDONE where 2 is index
save     - Save list to file
load     - Load list from file";

    ActionResult::Feedback(feedback.to_string())
}

fn list(l: &mut List) -> ActionResult {
    ActionResult::Feedback(l.to_text().to_string())
}

fn add(state: &mut State) -> ActionResult {
    state.set(C_ADD, Status::NeedPlainText, None);
    ActionResult::NeedTask
}

fn add_text(text: String, list: &mut List, state: &mut State) -> ActionResult {
    match list.add(text) {
        Ok(()) => {
            state.reset();
            ActionResult::Ok
        }
        Err(e) => map_list_error(e),
    }
}

fn edit(parse_result: ParseResult, list: &mut List, state: &mut State) -> ActionResult {
    if !utils::are_strings_integers(&parse_result.arguments) {
        return ActionResult::InvalidArguments;
    }

    let index = self::extract_index_from_args(&parse_result.arguments);

    match list.get(index) {
        Ok(..) => {
            state.set(C_EDIT, Status::NeedPlainText, Some(index));
            ActionResult::NeedTask
        }
        Err(e) => map_list_error(e),
    }
}

fn edit_text(raw_input: String, list: &mut List, state: &mut State) -> ActionResult {
    if let Some(index) = state.task_index {
        match list.alter(index, raw_input) {
            Ok(()) => {
                state.reset();
                ActionResult::Ok
            }
            Err(e) => map_list_error(e),
        }
    } else {
        ActionResult::TaskNotFound
    }
}

fn remove(parse_result: ParseResult, list: &mut List, state: &mut State) -> ActionResult {
    if !utils::are_strings_integers(&parse_result.arguments) {
        return ActionResult::InvalidArguments;
    }

    let index = self::extract_index_from_args(&parse_result.arguments);

    match list.get(index) {
        Ok(..) => {
            state.set(C_REMOVE, Status::NeedConfirmation, Some(index));
            ActionResult::NeedConfirm
        }
        Err(e) => map_list_error(e),
    }
}

fn remove_confirm(raw_input: String, list: &mut List, state: &mut State) -> ActionResult {
    if !command_parser::is_confirm(&raw_input) {
        return ActionResult::Sh;
    }

    if let Some(index) = state.task_index {
        match list.remove(index) {
            Ok(()) => {
                state.reset();
                ActionResult::Ok
            }
            Err(e) => map_list_error(e),
        }
    } else {
        ActionResult::TaskNotFound
    }
}

fn done(parse_result: ParseResult, list: &mut List) -> ActionResult {
    if !utils::are_strings_integers(&parse_result.arguments) {
        return ActionResult::InvalidArguments;
    }

    let index = self::extract_index_from_args(&parse_result.arguments);

    match list.mark_done(index) {
        Ok(()) => ActionResult::Ok,
        Err(e) => map_list_error(e),
    }
}

fn undone(parse_result: ParseResult, list: &mut List) -> ActionResult {
    if !utils::are_strings_integers(&parse_result.arguments) {
        return ActionResult::InvalidArguments;
    }

    let index = self::extract_index_from_args(&parse_result.arguments);

    match list.mark_undone(index) {
        Ok(()) => ActionResult::Ok,
        Err(e) => map_list_error(e),
    }
}

fn clear(list: &mut List, state: &mut State) -> ActionResult {
    if list.is_empty() {
        ActionResult::ListEmpty
    } else {
        println!();
        state.set(C_CLEAR, Status::NeedConfirmation, None);
        ActionResult::NeedConfirm
    }
}

fn clear_confirm(raw_input: String, list: &mut List, state: &mut State) -> ActionResult {
    state.reset();

    if command_parser::is_confirm(&raw_input) {
        list.clear();
        ActionResult::Ok
    } else {
        ActionResult::Sh
    }
}

fn save(list: &mut List, state: &mut State) -> ActionResult {
    if list.is_empty() {
        ActionResult::ListEmpty
    } else {
        state.set(C_SAVE, Status::NeedPlainText, None);
        ActionResult::NeedFilePath
    }
}

fn save_text(raw_input: String, list: &mut List, state: &mut State) -> ActionResult {
    let mut result = ActionResult::Ok;

    match fs::write(raw_input, list.to_text().as_bytes()) {
        Ok(..) => {
            state.reset();
        }
        Err(..) => {
            result = ActionResult::CannotSave;
        }
    }

    result
}

fn load(state: &mut State) -> ActionResult {
    state.set(C_LOAD, Status::NeedPlainText, None);
    ActionResult::NeedFilePath
}

fn load_text(raw_input: String, list: &mut List, state: &mut State) -> ActionResult {
    let mut result = ActionResult::Ok;

    match fs::read_to_string(raw_input) {
        Result::Ok(contents) => match list.from_text(&contents) {
            Ok(..) => {
                state.reset();
            }
            Err(e) => {
                result = map_list_error(e);
            }
        },
        Err(..) => {
            result = ActionResult::CannotLoad;
        }
    }

    result
}

// =========== Process Action ===========

pub fn process(input: String, list: &mut List, state: &mut State) -> ActionResult {
    if state.status.is_some() {
        let raw_input = utils::trim_str(&input);

        match state.status {
            Some(Status::NeedPlainText) => match state.command {
                Some(C_ADD) => self::add_text(raw_input, list, state),
                Some(C_EDIT) => self::edit_text(raw_input, list, state),
                Some(C_SAVE) => self::save_text(raw_input, list, state),
                Some(C_LOAD) => self::load_text(raw_input, list, state),
                _ => ActionResult::Sh,
            },
            Some(Status::NeedConfirmation) => match state.command {
                Some(C_REMOVE) => self::remove_confirm(raw_input, list, state),
                Some(C_CLEAR) => self::clear_confirm(raw_input, list, state),
                _ => ActionResult::Sh,
            },
            None => ActionResult::Sh,
        }
    } else {
        let parse_result = command_parser::parse(&input);

        match parse_result.command {
            C_EXIT => self::exit(),
            C_HELP => self::help(),
            C_LIST => self::list(list),
            C_ADD => self::add(state),
            C_EDIT => self::edit(parse_result, list, state),
            C_REMOVE => self::remove(parse_result, list, state),
            C_DONE => self::done(parse_result, list),
            C_UNDONE => self::undone(parse_result, list),
            C_CLEAR => self::clear(list, state),
            C_SAVE => self::save(list, state),
            C_LOAD => self::load(state),
            _ => ActionResult::UnknownCommand,
        }
    }
}
