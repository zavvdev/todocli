#[derive(PartialEq)]
pub enum Status {
    NeedPlainText,
    NeedConfirmation,
}

pub struct State<'a> {
    pub command: Option<&'a str>,
    pub status: Option<Status>,
    pub task_index: Option<usize>,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        Self {
            command: None,
            status: None,
            task_index: None,
        }
    }

    pub fn set(&mut self, command: &'a str, status: Status, task_index: Option<usize>) {
        self.command = Some(command);
        self.status = Some(status);
        self.task_index = task_index;
    }

    pub fn reset(&mut self) {
        self.command = None;
        self.status = None;
        self.task_index = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_instance() {
        let state = State::new();

        assert!(state.command.is_none());
        assert!(state.status.is_none());
        assert!(state.task_index.is_none());
    }

    #[test]
    fn test_set() {
        let mut state = State::new();

        let command = "test";
        let task_index = 42;

        assert!(state.command.is_none());
        assert!(state.status.is_none());
        assert!(state.task_index.is_none());

        state.set(command, Status::NeedPlainText, Some(task_index));

        assert!(state.command == Some(command));
        assert!(state.status == Some(Status::NeedPlainText));
        assert!(state.task_index == Some(task_index));
    }

    #[test]
    fn test_reset() {
        let mut state = State::new();

        let command = "test";
        let task_index = 42;

        state.set(command, Status::NeedPlainText, Some(task_index));

        assert!(state.command == Some(command));
        assert!(state.status == Some(Status::NeedPlainText));
        assert!(state.task_index == Some(task_index));

        state.reset();

        assert!(state.command.is_none());
        assert!(state.status.is_none());
        assert!(state.task_index.is_none());
    }
}
