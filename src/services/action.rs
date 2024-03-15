use crate::{
    config::Command,
    models::{
        list::List,
        state::{State, Status},
    },
};

pub fn unknown_command() {
    println!("Unknown command");
}

pub fn invalid_arguments() {
    println!("Invalid arguments");
}

pub fn exit() {
    println!("Bye!");
}

pub fn list(l: &mut List) {
    // TODO: Show done/undone status
    for (index, task) in l.dump().iter().enumerate() {
        println!("{}) {}", index + 1, task.text);
    }
}

pub fn add(state: &mut State) {
    state.set(Command::Add, Status::NeedMoreData);
}

pub fn help() {}

pub fn edit() {}

pub fn done() {}

pub fn undone() {}

pub fn save() {}

pub fn load() {}

pub fn clear() {}

pub fn remove() {}
