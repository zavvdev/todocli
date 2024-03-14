mod config;
mod handlers;
mod models;
mod parsers;

use crate::config::ProcessResult;
use crate::handlers::input;
use crate::models::list::List;
use crate::models::state::State;

fn main() {
    let mut list = List::new();
    let mut state = State::new();

    loop {
        print!("> ");

        match input::process(&input::accept(), &mut list, &mut state) {
            ProcessResult::Exit => break,
            ProcessResult::UnknownCommand => {
                println!("Unknown command");
            }
            ProcessResult::Ok => continue,
        }
    }
}
