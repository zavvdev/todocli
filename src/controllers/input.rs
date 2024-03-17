use crate::config::{
    C_ADD, C_CLEAR, C_DONE, C_EDIT, C_EXIT, C_HELP, C_LIST, C_LOAD, C_REMOVE, C_SAVE, C_UNDONE,
};
use crate::controllers::action;
use crate::models::state::{State, Status};
use crate::parsers::command_parser;
use crate::{config::ProcessResult, models::list::List};
use std::io::{self, Write};

pub fn accept() -> String {
    let mut input = String::new();
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

pub fn process(input: String, list: &mut List, state: &mut State) -> ProcessResult {
    if state.status.is_some() {
        let raw_input = command_parser::parse_raw(input);

        match state.status {
            Some(Status::NeedPlainText) => match state.command {
                Some(C_ADD) => action::add_text(raw_input, list, state),
                Some(C_EDIT) => action::edit_text(raw_input, list, state),
                _ => ProcessResult::Ok,
            },
            Some(Status::NeedConfirmation) => match state.command {
                Some(C_REMOVE) => action::remove_confirm(raw_input, list, state),
                Some(C_CLEAR) => action::clear_confirm(raw_input, list, state),
                _ => ProcessResult::Ok,
            },
            None => ProcessResult::Ok,
        }
    } else {
        let parse_result = command_parser::parse(&input);

        match parse_result.command {
            C_EXIT => action::exit(),
            C_HELP => action::help(),
            C_LIST => action::list(list),
            C_ADD => action::add(state),
            C_EDIT => action::edit(parse_result, list, state),
            C_REMOVE => action::remove(parse_result, list, state),
            C_DONE => action::done(parse_result, list),
            C_UNDONE => action::undone(parse_result, list),
            C_CLEAR => action::clear(state),
            C_SAVE => action::save(),
            C_LOAD => action::load(),
            _ => {
                println!("Unknown command");
                ProcessResult::Ok
            }
        }
    }
}
