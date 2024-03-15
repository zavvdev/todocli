use crate::config::*;
use crate::models::task::Task;

pub enum ErrorCause {
    CapacityExceeded,
    NotFound,
}

pub struct List<'a> {
    tasks: Vec<Task<'a>>,
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        Self {
            tasks: Vec::with_capacity(TASKS_LIST_MAX_CAPACITY),
        }
    }

    pub fn add(&mut self, text: &'a str) -> Result<(), ErrorCause> {
        if self.tasks.len() < self.tasks.capacity() {
            self.tasks.push(Task::new(text));
            return Ok(());
        }

        Err(ErrorCause::CapacityExceeded)
    }

    pub fn remove(&mut self, index: usize) -> Result<(), ErrorCause> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            return Ok(());
        }

        Err(ErrorCause::NotFound)
    }

    pub fn alter(&mut self, index: usize, next_text: &'a str) -> Result<(), ErrorCause> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.alter(next_text);
                Ok(())
            }
            None => Err(ErrorCause::NotFound),
        }
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn mark_done(&mut self, index: usize) -> Result<(), ErrorCause> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.done();
                Ok(())
            }
            None => Err(ErrorCause::NotFound),
        }
    }

    pub fn mark_undone(&mut self, index: usize) -> Result<(), ErrorCause> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.undone();
                Ok(())
            }
            None => Err(ErrorCause::NotFound),
        }
    }

    pub fn dump(&mut self) -> &Vec<Task<'a>> {
        &self.tasks
    }
}
