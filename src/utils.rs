use regex::Regex;

pub fn is_arguments_integer(arguments: &Vec<&str>) -> bool {
    let re = Regex::new(r"[1-9]+").unwrap();

    if arguments.len() > 0 {
        for argument in arguments.iter() {
            if !re.is_match(argument) {
                return false;
            }
        }

        return true;
    }

    false
}

pub fn trim_str(target: &str) -> String {
    let entries: Vec<&str> = target.split_whitespace().collect();
    entries.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_arguments_integer_true() {
        assert!(is_arguments_integer(&vec!["1", "2", "42"]));
    }

    #[test]
    fn test_is_arguments_integer_false() {
        assert!(!is_arguments_integer(&vec!["1", "two", "42"]));
    }

    #[test]
    fn test_trim_str() {
        assert_eq!(trim_str("    hello   world!  "), "hello world!");
    }
}
