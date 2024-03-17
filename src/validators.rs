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
