use crate::config::{
    C_ADD, C_CLEAR, C_DONE, C_EDIT, C_EXIT, C_HELP, C_LIST, C_LOAD, C_REMOVE, C_SAVE, C_UNDONE,
};
use crate::models::state::State;
use crate::parsers::command_parser;
use crate::services::action;
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

pub fn process(input: &str, list: &mut List, state: &mut State) -> ProcessResult {
    let parse_result = command_parser::parse(input);

    // TODO: react to state's status first

    match parse_result.command {
        C_EXIT => action::exit(),
        C_HELP => action::help(),
        C_LIST => action::list(list),
        C_ADD => action::add(state),
        C_EDIT => {
            // TODO: Validate arguments
            
            action::edit(
                parse_result
                    .arguments
                    .first()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            )
        }
        C_REMOVE => action::remove(),
        C_DONE => action::done(),
        C_UNDONE => action::undone(),
        C_SAVE => action::save(),
        C_LOAD => action::load(),
        C_CLEAR => action::clear(),
        _ => action::unknown_command(),
    }
}
