use crate::config::{Command, C_EXIT, C_LIST};

pub enum ParseError {
    InvalidCommand,
    InvalidArguments,
}

struct DecomposeResult<'a> {
    command: &'a str,
    arguments: Vec<&'a str>,
}

pub struct ParseResult<'a> {
    pub command: Option<Command>,
    pub arguments: Vec<&'a str>,
    pub error: Option<ParseError>,
}

fn decompose<'a>(target: &'a str) -> DecomposeResult<'a> {
    let entries: Vec<&str> = target.split_whitespace().collect();

    if entries.len() > 0 {
        return DecomposeResult {
            command: match entries.get(0) {
                Some(c) => c,
                None => "",
            },
            arguments: entries[1..].to_vec(),
        };
    }

    DecomposeResult {
        command: "",
        arguments: vec![],
    }
}

pub fn parse(target: &str) -> ParseResult {
    let decomposed = self::decompose(target);

    match decomposed.command {
        C_EXIT => ParseResult {
            command: Some(Command::Exit),
            arguments: vec![],
            error: None,
        },
        C_LIST => ParseResult {
            command: Some(Command::List),
            arguments: vec![],
            error: None,
        },
        // TODO: Implement parsing for all commands
        _ => ParseResult {
            command: None,
            arguments: vec![],
            error: Some(ParseError::InvalidCommand),
        },
    }
}
