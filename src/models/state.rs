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
