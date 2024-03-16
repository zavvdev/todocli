pub enum Status {
    NeedMoreData,
}

pub struct State<'a> {
    command: Option<&'a str>,
    status: Option<Status>,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        Self {
            command: None,
            status: None,
        }
    }

    pub fn set(&mut self, command: &'a str, status: Status) {
        self.command = Some(command);
        self.status = Some(status);
    }

    pub fn reset(&mut self) {
        self.command = None;
        self.status = None;
    }
}
