use crate::config::{Command, Status};

pub struct State {
    command: Option<Command>,
    status: Option<Status>,
}

impl State {
    pub fn new() -> Self {
        Self {
            command: None,
            status: None,
        }
    }

    pub fn set(&mut self, command: Command, status: Status) {
        self.command = Some(command);
        self.status = Some(status);
    }

    pub fn reset(&mut self) {
        self.command = None;
        self.status = None;
    }
}
