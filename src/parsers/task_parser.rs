use crate::models::task::Task;

pub fn to_text(tasks: &Vec<Task>) -> String {
    let mut result = String::new();

    for (index, task) in tasks.iter().enumerate() {
        let status = match task.is_done {
            true => "[+]",
            false => "[ ]",
        };

        result.push_str(&format!("{}) {} {}\n", index + 1, status, task.text));
    }

    result
}
