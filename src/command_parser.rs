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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_confirm() {
        assert!(is_confirm("y"));
        assert!(is_confirm("Y"));
        assert!(is_confirm("yes"));
        assert!(is_confirm("Yes"));
        assert!(is_confirm("YES"));
    }

    #[test]
    fn test_parse_empty() {
        let result = parse("");
        assert!(result.command == "");
        assert!(result.arguments.len() == 0);
    }

    #[test]
    fn test_parse_command() {
        let command = "hello";
        let result = parse(command);
        assert!(result.command == command);
    }

    #[test]
    fn test_parse_with_arguments() {
        let arguments = vec!["a", "b"];
        let result = parse("hello a b");
        assert!(result.arguments.get(0) == arguments.get(0));
        assert!(result.arguments.get(1) == arguments.get(1));
    }
}
