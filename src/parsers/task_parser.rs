use regex::Regex;

use crate::{models::task::Task, utils};

const DONE: &str = "[+]";
const UNDONE: &str = "[ ]";

pub fn to_text(tasks: &Vec<Task>) -> String {
    let mut result = String::new();

    for (index, task) in tasks.iter().enumerate() {
        let status = match task.is_done {
            true => DONE,
            false => UNDONE,
        };

        result.push_str(&format!("{}) {} {};\n", index + 1, status, task.text));
    }

    result
}

pub fn from_text(text: &str) -> Result<Vec<Task>, ()> {
    let mut result: Vec<Task> = Vec::new();
    let re = Regex::new(r"(\[\s*\+?\s*\])(([^;\[\]])+)(;)").unwrap();

    if re.is_match(&text) {
        for c in re.captures_iter(text) {
            let check = utils::trim_str(c.get(1).unwrap().into());
            let text = utils::trim_str(c.get(2).unwrap().into());

            result.push(Task {
                text,
                is_done: check == DONE,
            });
        }

        Ok(result)
    } else {
        Err(())
    }
}
