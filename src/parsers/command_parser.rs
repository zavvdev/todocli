use crate::config::{C_Y, C_YES};

pub struct ParseResult<'a> {
    pub command: &'a str,
    pub arguments: Vec<&'a str>,
}

pub fn is_confirm(target: &str) -> bool {
    let target = target.to_lowercase();
    target == C_Y || target == C_YES
}

pub fn parse<'a>(target: &'a str) -> ParseResult<'a> {
    let entries: Vec<&str> = target.split_whitespace().collect();

    if entries.len() > 0 {
        return ParseResult {
            command: match entries.get(0) {
                Some(c) => c,
                None => "",
            },
            arguments: entries[1..].to_vec(),
        };
    }

    ParseResult {
        command: "",
        arguments: vec![],
    }
}
