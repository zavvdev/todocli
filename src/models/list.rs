use crate::config::*;
use crate::models::task::Task;

pub struct List {
    tasks: Vec<Task>,
}

impl List {
    pub fn new() -> Self {
        Self {
            tasks: Vec::with_capacity(TASKS_LIST_MAX_CAPACITY),
        }
    }

    pub fn get(&mut self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }

    pub fn add(&mut self, text: String) -> Result<(), ProcessError> {
        if self.tasks.len() < self.tasks.capacity() {
            self.tasks.push(Task::new(text));
            return Ok(());
        }

        Err(ProcessError::ListCapacityExceeded)
    }

    pub fn remove(&mut self, index: usize) -> Result<(), ProcessError> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            return Ok(());
        }

        Err(ProcessError::ListItemNotFound)
    }

    pub fn alter(&mut self, index: usize, next_text: String) -> Result<(), ProcessError> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.alter(next_text);
                Ok(())
            }
            None => Err(ProcessError::ListItemNotFound),
        }
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn mark_done(&mut self, index: usize) -> Result<(), ProcessError> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.done();
                Ok(())
            }
            None => Err(ProcessError::ListItemNotFound),
        }
    }

    pub fn mark_undone(&mut self, index: usize) -> Result<(), ProcessError> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.undone();
                Ok(())
            }
            None => Err(ProcessError::ListItemNotFound),
        }
    }

    pub fn dump(&mut self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn populate(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
    }

    pub fn is_empty(&mut self) -> bool {
        self.tasks.is_empty()
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
        assert!(result.is_some());
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
        assert!(list.get(0).unwrap().text == "new_text");
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
    fn test_dump() {
        let mut list = List::new();
        let _ = list.add("test".to_string());
        let result = list.dump();

        assert!(result.len() == 1);
        assert!(result.get(0).unwrap().text == "test");
    }

    #[test]
    fn test_populate() {
        let mut list = List::new();
        list.populate(vec![Task {
            text: "test".to_string(),
            is_done: false,
        }]);

        assert!(list.tasks.len() == 1);
        assert!(list.get(0).unwrap().text == "test");
    }

    #[test]
    fn test_is_empty() {
        let mut list = List::new();
        assert!(list.is_empty());
    }
}
