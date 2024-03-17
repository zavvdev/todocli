pub struct ParseResult<'a> {
    pub command: &'a str,
    pub arguments: Vec<&'a str>,
}

pub fn prepare_input(target: String) -> String {
    let entries: Vec<&str> = target.split_whitespace().collect();
    entries.join(" ")
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
