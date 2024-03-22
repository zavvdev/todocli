mod command_parser;
mod config;
mod controllers;
mod models;
mod utils;

use crate::controllers::{
    action::{self, ActionResult},
    input,
};
use crate::models::list::List;
use crate::models::state::State;

pub fn run() {
    let mut list = List::new();
    let mut state = State::new();

    println!("-------todocli-------");

    loop {
        print!("> ");

        match action::process(input::accept(), &mut list, &mut state) {
            ActionResult::Sh => continue,

            ActionResult::Ok => {
                println!("ok.");
                continue;
            }

            ActionResult::Terminate => {
                println!("bye!");
                break;
            }

            ActionResult::ListFull => {
                println!("list full");
            }

            ActionResult::ListEmpty => {
                println!("list empty");
            }

            ActionResult::TaskNotFound => {
                println!("task not found");
            }

            ActionResult::FileReadError => {
                println!("file read error (cannot parse)");
            }

            ActionResult::UnknownCommand => {
                println!("unknown command");
            }

            ActionResult::InvalidArguments => {
                println!("invalid arguments");
            }

            ActionResult::NeedConfirm => {
                println!("confirm? (y/n)");
            }

            ActionResult::NeedFilePath => {
                println!("provide file path");
            }

            ActionResult::NeedTask => {
                println!("enter task:");
            }

            ActionResult::CannotSave => {
                println!("cannot save");
            }

            ActionResult::CannotLoad => {
                println!("cannot load");
            }

            ActionResult::Feedback(feedback) => println!("{feedback}"),
        }
    }
}
