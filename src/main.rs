mod config;
mod controllers;
mod models;
mod parsers;
mod services;

use crate::controllers::input;
use crate::config::ProcessResult;
use crate::models::list::List;
use crate::models::state::State;

fn main() {
    let mut list = List::new();
    let mut state = State::new();

    loop {
        print!("> ");

        match input::process(&input::accept(), &mut list, &mut state) {
            ProcessResult::Ok => continue,
            ProcessResult::Terminate => break,
            ProcessResult::Error => println!("Something went wrong..."),
        }
    }
}
