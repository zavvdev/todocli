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
}
