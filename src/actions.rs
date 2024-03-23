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

#[derive(Debug, PartialEq)]
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

// =========== Helpers ===========

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
    let feedback = format!(
        "{C_EXIT}     - Exit program
{C_HELP}     - View available commands
{C_LIST}     - View all tasks
{C_CLEAR}    - Clear tasks
{C_ADD}      - Add new task
{C_EDIT} 2   - Edit task by index where 2 is index
{C_REMOVE} 2 - Delete task by index where 2 is index
{C_DONE} 2   - Mark task as DONE where 2 is index
{C_UNDONE} 2 - Mark task as UNDONE where 2 is index
{C_SAVE}     - Save list to file
{C_LOAD}     - Load list from file"
    );

    ActionResult::Feedback(feedback.to_string())
}

fn list(l: &mut List) -> ActionResult {
    ActionResult::Feedback(l.to_text())
}

fn add(state: &mut State) -> ActionResult {
    state.set(C_ADD, Status::NeedPlainText, None);
    ActionResult::NeedTask
}

fn add_text(text: String, list: &mut List, state: &mut State) -> ActionResult {
    state.reset();

    match list.add(text) {
        Ok(()) => ActionResult::Ok,
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
        state.reset();

        match list.alter(index, raw_input) {
            Ok(()) => ActionResult::Ok,
            Err(e) => map_list_error(e),
        }
    } else {
        state.reset();
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
        state.reset();
        return ActionResult::Sh;
    }

    if let Some(index) = state.task_index {
        state.reset();

        match list.remove(index) {
            Ok(()) => ActionResult::Ok,
            Err(e) => map_list_error(e),
        }
    } else {
        state.reset();
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
    state.reset();

    match fs::write(raw_input, list.to_text().as_bytes()) {
        Ok(..) => ActionResult::Ok,
        Err(..) => ActionResult::CannotSave,
    }
}

fn load(state: &mut State) -> ActionResult {
    state.set(C_LOAD, Status::NeedPlainText, None);
    ActionResult::NeedFilePath
}

fn load_text(raw_input: String, list: &mut List, state: &mut State) -> ActionResult {
    state.reset();

    match fs::read_to_string(raw_input) {
        Result::Ok(contents) => match list.from_text(&contents) {
            Ok(..) => ActionResult::Ok,
            Err(e) => map_list_error(e),
        },
        Err(..) => ActionResult::CannotLoad,
    }
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

// =========== Tests ===========

#[cfg(test)]
mod tests {
    use crate::config::{C_Y, C_YES};

    use super::*;

    #[test]
    fn test_process_exit() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(C_EXIT.to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Terminate);
    }

    #[test]
    fn test_process_help() {
        let mut list = List::new();
        let mut state = State::new();

        let expected = format!(
            "{C_EXIT}     - Exit program
{C_HELP}     - View available commands
{C_LIST}     - View all tasks
{C_CLEAR}    - Clear tasks
{C_ADD}      - Add new task
{C_EDIT} 2   - Edit task by index where 2 is index
{C_REMOVE} 2 - Delete task by index where 2 is index
{C_DONE} 2   - Mark task as DONE where 2 is index
{C_UNDONE} 2 - Mark task as UNDONE where 2 is index
{C_SAVE}     - Save list to file
{C_LOAD}     - Load list from file"
        );

        let result = process(C_HELP.to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Feedback(expected));
    }

    #[test]
    fn test_process_list_empty() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(C_LIST.to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Feedback("".to_string()));
    }

    #[test]
    fn test_process_list() {
        let mut list = List::new();
        let mut state = State::new();

        let _ = list.add("test1".to_string());
        let _ = list.add("test2".to_string());

        let expected = "1) [ ] test1;\n2) [ ] test2;\n".to_string();
        let result = process(C_LIST.to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Feedback(expected));
    }

    #[test]
    fn test_process_add() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(C_ADD.to_string(), &mut list, &mut state);

        assert!(list.is_empty());
        assert_eq!(result, ActionResult::NeedTask);
        assert_eq!(state.status, Some(Status::NeedPlainText));
        assert_eq!(state.command, Some(C_ADD));
        assert_eq!(state.task_index, None);

        state.set(C_ADD, Status::NeedPlainText, None);

        let task = "test";
        let result = process(task.to_string(), &mut list, &mut state);

        assert!(list.get(0).unwrap().text == task);
        assert_eq!(result, ActionResult::Ok);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_edit() {
        let mut list = List::new();
        let mut state = State::new();

        let task_before_edit = "test";
        let task_after_edit = "new-test";

        let _ = list.add(task_before_edit.to_string());

        assert!(list.get(0).unwrap().text == task_before_edit);
        let result = process(format!("{C_EDIT} 1"), &mut list, &mut state);

        assert_eq!(result, ActionResult::NeedTask);
        assert_eq!(state.status, Some(Status::NeedPlainText));
        assert_eq!(state.command, Some(C_EDIT));
        assert_eq!(state.task_index, Some(0));

        let result = process(task_after_edit.to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Ok);
        assert!(list.get(0).unwrap().text == task_after_edit);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_edit_not_found() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(format!("{C_EDIT} 1"), &mut list, &mut state);

        assert_eq!(result, ActionResult::TaskNotFound);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_edit_invalid() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(format!("{C_EDIT} test"), &mut list, &mut state);

        assert_eq!(result, ActionResult::InvalidArguments);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_remove() {
        let mut list = List::new();
        let mut state = State::new();

        let _ = list.add("test".to_string());

        assert!(!list.is_empty());
        let result = process(format!("{C_REMOVE} 1"), &mut list, &mut state);

        assert_eq!(result, ActionResult::NeedConfirm);
        assert_eq!(state.command, Some(C_REMOVE));
        assert_eq!(state.status, Some(Status::NeedConfirmation));
        assert_eq!(state.task_index, Some(0));

        let result = process(C_YES.to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Ok);
        assert!(list.is_empty());
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_remove_decline() {
        let mut list = List::new();
        let mut state = State::new();
        let task = "test";

        let _ = list.add(task.to_string());

        assert!(list.get(0).unwrap().text == task);
        let result = process(format!("{C_REMOVE} 1"), &mut list, &mut state);

        assert_eq!(result, ActionResult::NeedConfirm);
        assert_eq!(state.command, Some(C_REMOVE));
        assert_eq!(state.status, Some(Status::NeedConfirmation));
        assert_eq!(state.task_index, Some(0));

        let result = process("n".to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Sh);
        assert!(list.get(0).unwrap().text == task);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_remove_not_found() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(format!("{C_REMOVE} 1"), &mut list, &mut state);

        assert_eq!(result, ActionResult::TaskNotFound);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_remove_invalid() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(format!("{C_REMOVE} 1 two"), &mut list, &mut state);

        assert_eq!(result, ActionResult::InvalidArguments);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_done() {
        let mut list = List::new();
        let mut state = State::new();

        let _ = list.add("test".to_string());

        assert!(!list.get(0).unwrap().is_done);

        let result = process(format!("{C_DONE} 1"), &mut list, &mut state);

        assert!(list.get(0).unwrap().is_done);
        assert_eq!(result, ActionResult::Ok);
    }

    #[test]
    fn test_process_done_not_found() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(format!("{C_DONE} 1"), &mut list, &mut state);

        assert_eq!(result, ActionResult::TaskNotFound);
    }

    #[test]
    fn test_process_undone() {
        let mut list = List::new();
        let mut state = State::new();

        let _ = list.add("test".to_string());
        let _ = list.mark_done(0);

        assert!(list.get(0).unwrap().is_done);

        let result = process(format!("{C_UNDONE} 1"), &mut list, &mut state);

        assert!(!list.get(0).unwrap().is_done);
        assert_eq!(result, ActionResult::Ok);
    }

    #[test]
    fn test_process_undone_not_found() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(format!("{C_UNDONE} 1"), &mut list, &mut state);

        assert_eq!(result, ActionResult::TaskNotFound);
    }

    #[test]
    fn test_process_clear() {
        let mut list = List::new();
        let mut state = State::new();

        let _ = list.add("test".to_string());

        assert!(!list.is_empty());

        let result = process(format!("{C_CLEAR}"), &mut list, &mut state);

        assert_eq!(result, ActionResult::NeedConfirm);
        assert_eq!(state.command, Some(C_CLEAR));
        assert_eq!(state.status, Some(Status::NeedConfirmation));
        assert_eq!(state.task_index, None);

        let result = process(C_Y.to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Ok);
        assert!(list.is_empty());
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_clear_decline() {
        let mut list = List::new();
        let mut state = State::new();

        let _ = list.add("test".to_string());

        assert!(!list.is_empty());

        let result = process(format!("{C_CLEAR}"), &mut list, &mut state);

        assert_eq!(result, ActionResult::NeedConfirm);
        assert_eq!(state.command, Some(C_CLEAR));
        assert_eq!(state.status, Some(Status::NeedConfirmation));
        assert_eq!(state.task_index, None);

        let result = process("no".to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Sh);
        assert!(!list.is_empty());
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_clear_empty() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(format!("{C_CLEAR}"), &mut list, &mut state);

        assert_eq!(result, ActionResult::ListEmpty);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_save() {
        let mut list = List::new();
        let mut state = State::new();

        let _ = list.add("learn rust".to_string());
        let _ = list.add("learn javascript".to_string());
        let _ = list.mark_done(1);

        let result = process(format!("{C_SAVE}"), &mut list, &mut state);

        assert_eq!(result, ActionResult::NeedFilePath);
        assert_eq!(state.command, Some(C_SAVE));
        assert_eq!(state.status, Some(Status::NeedPlainText));
        assert_eq!(state.task_index, None);

        let result = process("./test_process_save.txt".to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Ok);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_save_empty() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(format!("{C_SAVE}"), &mut list, &mut state);

        assert_eq!(result, ActionResult::ListEmpty);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_process_load() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(format!("{C_LOAD}"), &mut list, &mut state);

        assert_eq!(result, ActionResult::NeedFilePath);
        assert_eq!(state.command, Some(C_LOAD));
        assert_eq!(state.status, Some(Status::NeedPlainText));
        assert_eq!(state.task_index, None);

        let result = process("./test_process_save.txt".to_string(), &mut list, &mut state);

        assert_eq!(result, ActionResult::Ok);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());

        assert!(list.get(0).unwrap().text == "learn rust");
        assert!(list.get(1).unwrap().text == "learn javascript");
        assert!(!list.get(0).unwrap().is_done);
        assert!(list.get(1).unwrap().is_done);
    }

    #[test]
    fn test_process_load_invalid() {
        let mut list = List::new();
        let mut state = State::new();

        let result = process(format!("{C_LOAD}"), &mut list, &mut state);

        assert_eq!(result, ActionResult::NeedFilePath);
        assert_eq!(state.command, Some(C_LOAD));
        assert_eq!(state.status, Some(Status::NeedPlainText));
        assert_eq!(state.task_index, None);

        let result = process(
            "./test_process_load_invalid.txt".to_string(),
            &mut list,
            &mut state,
        );

        assert_eq!(result, ActionResult::FileReadError);
        assert!(state.status.is_none());
        assert!(state.command.is_none());
        assert!(state.task_index.is_none());

        assert!(list.is_empty());
    }
}
