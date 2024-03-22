use crate::{config::TASKS_LIST_MAX_CAPACITY, utils};
use regex::Regex;

const DONE_MARK: &str = "[+]";
const UNDONE_MARK: &str = "[ ]";

pub enum Error {
    CapacityExceeded,
    ItemNotFound,
    InvalidPattern,
}

pub struct Task {
    pub text: String,
    pub is_done: bool,
}

pub struct List {
    tasks: Vec<Task>,
}

impl List {
    pub fn new() -> Self {
        Self {
            tasks: Vec::with_capacity(TASKS_LIST_MAX_CAPACITY),
        }
    }

    pub fn get(&mut self, index: usize) -> Result<&Task, Error> {
        match self.tasks.get(index) {
            Some(task) => Ok(&task),
            None => Err(Error::ItemNotFound),
        }
    }

    pub fn add(&mut self, text: String) -> Result<(), Error> {
        if self.tasks.len() < self.tasks.capacity() {
            self.tasks.push(Task {
                text,
                is_done: false,
            });
            return Ok(());
        }

        Err(Error::CapacityExceeded)
    }

    pub fn remove(&mut self, index: usize) -> Result<(), Error> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            return Ok(());
        }

        Err(Error::ItemNotFound)
    }

    pub fn alter(&mut self, index: usize, next_text: String) -> Result<(), Error> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.text = next_text;
                Ok(())
            }
            None => Err(Error::ItemNotFound),
        }
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn mark_done(&mut self, index: usize) -> Result<(), Error> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.is_done = true;
                Ok(())
            }
            None => Err(Error::ItemNotFound),
        }
    }

    pub fn mark_undone(&mut self, index: usize) -> Result<(), Error> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.is_done = false;
                Ok(())
            }
            None => Err(Error::ItemNotFound),
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.tasks.is_empty()
    }

    pub fn to_text(&self) -> String {
        let mut result = String::new();

        self.tasks.iter().enumerate().for_each(|(i, t)| {
            let status = match t.is_done {
                true => DONE_MARK,
                false => UNDONE_MARK,
            };

            result.push_str(&format!("{}) {} {};\n", i + 1, status, t.text));
        });

        result
    }

    pub fn from_text(&mut self, text: &str) -> Result<(), Error> {
        let mut result: Vec<Task> = Vec::new();
        let re = Regex::new(r"(\[\s*\+?\s*\])(([^;\[\]])+)(;)").unwrap();

        if re.is_match(&text) {
            re.captures_iter(text).for_each(|c| {
                let check = utils::trim_str(c.get(1).unwrap().into());
                let text = utils::trim_str(c.get(2).unwrap().into());

                result.push(Task {
                    text: text.to_string(),
                    is_done: check == DONE_MARK,
                });
            });

            self.tasks = result;
            Ok(())
        } else {
            Err(Error::InvalidPattern)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = List::new();
        assert!(list.tasks.len() == 0);
        assert!(list.tasks.capacity() == TASKS_LIST_MAX_CAPACITY);
    }

    #[test]
    fn test_get() {
        let mut list = List::new();
        let _ = list.add("test".to_string());
        let result = list.get(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add() {
        let mut list = List::new();
        let result = list.add("test".to_string());
        assert!(result.is_ok());
        assert!(list.tasks.len() == 1);
    }

    #[test]
    fn test_remove() {
        let mut list = List::new();
        let _ = list.add("test".to_string());
        assert!(list.tasks.len() == 1);
        let result = list.remove(0);
        assert!(result.is_ok());
        assert!(list.tasks.len() == 0);
    }

    #[test]
    fn test_alter() {
        let mut list = List::new();
        let _ = list.add("test1".to_string());
        assert!(list.get(0).unwrap().text == "test1");

        let result = list.alter(0, "new_text".to_string());
        assert!(result.is_ok());
        assert!(list.get(0).unwrap().text == "new_text".to_string());
    }

    #[test]
    fn test_clear() {
        let mut list = List::new();
        let _ = list.add("test".to_string());

        assert!(list.tasks.len() == 1);
        list.clear();
        assert!(list.tasks.is_empty());
    }

    #[test]
    fn test_mark_done() {
        let mut list = List::new();
        let _ = list.add("test".to_string());

        assert!(!list.get(0).unwrap().is_done);
        let result = list.mark_done(0);
        assert!(result.is_ok());
        assert!(list.get(0).unwrap().is_done);
    }

    #[test]
    fn test_mark_undone() {
        let mut list = List::new();
        let _ = list.add("test".to_string());
        let _ = list.mark_done(0);

        assert!(list.get(0).unwrap().is_done);
        let result = list.mark_undone(0);
        assert!(result.is_ok());
        assert!(!list.get(0).unwrap().is_done);
    }

    #[test]
    fn test_is_empty() {
        let mut list = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_to_text() {
        let mut list = List::new();
        let _ = list.add("test1".to_string());
        let _ = list.add("test2".to_string());
        let _ = list.mark_done(0);
        let result = list.to_text();

        assert!(result == "1) [+] test1;\n2) [ ] test2;\n");
    }

    #[test]
    fn test_from_text() {
        let mut list = List::new();

        let expected = vec![
            Task {
                text: "test1".to_string(),
                is_done: false,
            },
            Task {
                text: "test2".to_string(),
                is_done: true,
            },
        ];

        let _ = list.from_text("1) [ ] test1;\n2) [+] test2;\n");

        list.tasks.iter().enumerate().for_each(|(i, t)| {
            assert!(t.text == expected.get(i).unwrap().text);
            assert!(t.is_done == expected.get(i).unwrap().is_done);
        });
    }
}
