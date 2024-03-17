pub struct Task {
    pub text: String,
    pub is_done: bool,
}

impl Task {
    pub fn new(text: String) -> Self {
        Self {
            text,
            is_done: false,
        }
    }

    pub fn done(&mut self) {
        self.is_done = true;
    }

    pub fn undone(&mut self) {
        self.is_done = false;
    }

    pub fn alter(&mut self, next_text: String) {
        self.text = next_text;
    }
}
