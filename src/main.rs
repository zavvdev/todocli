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

    println!("-------todocli-------");

    loop {
        print!("> ");

        match input::process(input::accept(), &mut list, &mut state) {
            ProcessResult::Sh => continue,
            ProcessResult::Ok => {
                println!("ok.");
                continue;
            }
            ProcessResult::Feedback(feedback) => println!("{feedback}"),
            ProcessResult::Terminate => {
                println!("bye!");
                break;
            }
            ProcessResult::Error(cause) => {
                state.reset();
                match cause {
                    ProcessError::ListCapacityExceeded => println!("list capacity exceeded"),
                    ProcessError::ListItemNotFound => println!("list item not found"),
                    ProcessError::TaskIndexMissing => println!("missing"),
                    ProcessError::InvalidArguments => println!("invalid arguments"),
                    ProcessError::UnknownCommand => println!("unknown command"),
                    ProcessError::CannotCreateFile => println!("can't create file"),
                    ProcessError::CannotWriteToFile => println!("can't write to file"),
                }
            }
        }
    }
}
