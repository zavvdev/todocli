use std::io::{self, Write};

// exit            - terminate the program
// help            - display help message
// list            - list all items or options
// add [list name] - add a new item or option
// remove [...ids] - remove items or options
// done [...ids]   - mark items or options as done
// undone [...ids] - mark items as undone
// clear           - remove all items or options
// save            - save the current list to a file
// load            - load a list from a file

const EXIT: &str = "exit";

fn trim_whitespaces(s: &str) -> String {
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

fn exit() {
    println!("Bye!");
}

fn process_input(input: &str) -> bool {
    if input == EXIT {
        exit();
        return true;
    }

    false
}

fn prepare_input(s: &str) -> String {
    trim_whitespaces(s)
}

fn accept_input() -> String {
    let mut input = String::new();
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

fn main() {
    loop {
        print!("> ");

        if process_input(&prepare_input(&accept_input())) {
            break;
        }

        println!("Unknown command");
    }
}
