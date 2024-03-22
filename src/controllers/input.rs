use std::io::{self, Write};

pub fn accept() -> String {
    let mut input = String::new();
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input
}
