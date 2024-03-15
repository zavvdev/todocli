mod config;
mod models;
mod parsers;
mod controllers;
mod services;

use crate::controllers::input;
use crate::input::ProcessResult;
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
        }
    }
}
