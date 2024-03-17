use crate::config::{
    C_ADD, C_CLEAR, C_DONE, C_EDIT, C_EXIT, C_HELP, C_LIST, C_LOAD, C_REMOVE, C_SAVE, C_UNDONE,
};
use crate::models::state::{State, Status};
use crate::parsers::command_parser;
use crate::services::action;
use crate::validators;
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

fn invalid_arguments_result() -> ProcessResult {
    println!("Invalid arguments");
    ProcessResult::Ok
}

pub fn process(input: String, list: &mut List, state: &mut State) -> ProcessResult {
    if state.status.is_some() {
        let prepared_input = command_parser::prepare_input(input);

        match state.status {
            Some(Status::NeedPlainText) => match state.command {
                Some(C_ADD) => action::add_text(prepared_input, list, state),
                Some(C_EDIT) => action::edit_text(prepared_input, list, state),
                _ => ProcessResult::Ok,
            },
            Some(Status::NeedConfirmation) => ProcessResult::Ok,
            None => ProcessResult::Ok,
        }
    } else {
        let parse_result = command_parser::parse(&input);

        match parse_result.command {
            C_EXIT => action::exit(),

            C_HELP => action::help(),

            C_LIST => action::list(list),

            C_ADD => action::add(state),

            C_EDIT => {
                if validators::is_arguments_integer(&parse_result.arguments) {
                    let index = parse_result
                        .arguments
                        .first()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();

                    return action::edit(index - 1, list, state);
                }

                self::invalid_arguments_result()
            }

            C_REMOVE => {
                if validators::is_arguments_integer(&parse_result.arguments) {
                    return action::remove(
                        parse_result
                            .arguments
                            .first()
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    );
                }

                self::invalid_arguments_result()
            }

            C_DONE => {
                if validators::is_arguments_integer(&parse_result.arguments) {
                    return action::done(
                        parse_result
                            .arguments
                            .first()
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    );
                }

                self::invalid_arguments_result()
            }

            C_UNDONE => {
                if validators::is_arguments_integer(&parse_result.arguments) {
                    return action::undone(
                        parse_result
                            .arguments
                            .first()
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    );
                }

                self::invalid_arguments_result()
            }

            C_SAVE => action::save(),

            C_LOAD => action::load(),

            C_CLEAR => action::clear(),

            _ => {
                println!("Unknown command");
                ProcessResult::Ok
            }
        }
    }
}
