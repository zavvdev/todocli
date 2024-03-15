pub struct Task<'a> {
    pub text: &'a str,
    pub is_done: bool,
}

impl<'a> Task<'a> {
    pub fn new(text: &'a str) -> Self {
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

    pub fn alter(&mut self, next_text: &'a str) {
        self.text = next_text;
    }
}
