pub struct Task {
    text: String,
    is_done: bool,
}

impl Task {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            is_done: false,
        }
    }

    pub fn done(&mut self) {
        self.is_done = true;
    }

    pub fn undone(&mut self) {
        self.is_done = false;
    }

    pub fn alter(&mut self, next_text: &str) {
        self.text = next_text.to_string();
    }
}
