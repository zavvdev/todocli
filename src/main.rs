mod config;
mod controllers;
mod models;
mod parsers;
mod validators;

use crate::config::{ProcessError, ProcessResult};
use crate::controllers::input;
use crate::models::list::List;
use crate::models::state::State;

fn main() {
    let mut list = List::new();
    let mut state = State::new();
        
    println!("-------ToDo CLI-------");

    loop {
        print!("> ");

        match input::process(input::accept(), &mut list, &mut state) {
            ProcessResult::Ok => continue,
            ProcessResult::Terminate => break,
            ProcessResult::Error(cause) => {
                state.reset();
                match cause {
                    ProcessError::ListCapacityExceeded => println!("List capacity exceeded"),
                    ProcessError::ListItemNotFound => println!("List item not found"),
                    ProcessError::TaskIndexMissing => println!("Task index missing"),
                }
            }
        }
    }
}
