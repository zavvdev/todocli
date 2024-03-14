use crate::config::*;
use crate::models::list::List;
use crate::models::state::State;
use std::io::{self, Write};

pub fn accept() -> String {
    let mut input = String::new();
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

pub fn process(_input: &str, _list: &mut List, _state: &mut State) -> ProcessResult {
    // TODO: Implementation
    ProcessResult::UnknownCommand
}
