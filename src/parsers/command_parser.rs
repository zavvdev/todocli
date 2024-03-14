pub struct CommandParseResult<'a> {
    command: &'a str,
    arguments: Vec<&'a str>,
}

pub fn parse<'a>(target: &'a str) -> CommandParseResult<'a> {
    let entries: Vec<&str> = target.split_whitespace().collect();

    if entries.len() > 0 {
        return CommandParseResult {
            command: match entries.get(0) {
                Some(command) => command,
                None => "",
            },
            arguments: entries[1..].to_vec(),
        }
    }

    CommandParseResult {
        command: "",
        arguments: vec![],
    }

    // TODO: Implement parse for each command
}
