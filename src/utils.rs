use regex::Regex;
use std::io::{self, Write};

pub fn are_strings_integers(strings: &Vec<&str>) -> bool {
    let re = Regex::new(r"[1-9]+").unwrap();
    strings.iter().all(|s| re.is_match(s))
}

pub fn trim_str(target: &str) -> String {
    let entries: Vec<&str> = target.split_whitespace().collect();
    entries.join(" ")
}

pub fn get_user_input() -> String {
    let mut input = String::new();
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_strings_integers_true() {
        assert!(are_strings_integers(&vec!["1", "2", "42"]));
    }

    #[test]
    fn test_is_strings_integers_false() {
        assert!(!are_strings_integers(&vec!["1", "two", "42"]));
    }

    #[test]
    fn test_trim_str() {
        assert_eq!(trim_str("    hello   world!  "), "hello world!");
    }
}
