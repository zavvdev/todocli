use crate::models::state::State;
use crate::parsers::command_parser;
use crate::services::action;
use crate::{config::Command, models::list::List};
use std::io::{self, Write};

pub enum ProcessResult {
    Ok,
    Terminate,
}

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

    match parse_result.error {
        Some(command_parser::ParseError::InvalidCommand) => {
            action::unknown_command();
            ProcessResult::Ok
        }
        Some(command_parser::ParseError::InvalidArguments) => {
            action::invalid_arguments();
            ProcessResult::Ok
        }
        None => match parse_result.command {
            Some(Command::Exit) => {
                action::exit();
                ProcessResult::Terminate
            }
            Some(Command::List) => {
                action::list(list);
                ProcessResult::Ok
            }
            Some(Command::Add) => {
                action::add(state);
                ProcessResult::Ok
            }
            Some(Command::Help) => {
                action::help();
                ProcessResult::Ok
            }
            Some(Command::Edit) => {
                action::edit();
                ProcessResult::Ok
            }
            Some(Command::Done) => {
                action::done();
                ProcessResult::Ok
            }
            Some(Command::Undone) => {
                action::undone();
                ProcessResult::Ok
            }
            Some(Command::Save) => {
                action::save();
                ProcessResult::Ok
            }
            Some(Command::Load) => {
                action::load();
                ProcessResult::Ok
            }
            Some(Command::Clear) => {
                action::clear();
                ProcessResult::Ok
            }
            Some(Command::Remove) => {
                action::remove();
                ProcessResult::Ok
            }
            None => ProcessResult::Ok,
        },
    }
}
