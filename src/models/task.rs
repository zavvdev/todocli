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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_instance() {
        let text = String::from("initial text");
        let task = Task::new(text.clone());

        assert!(task.text == text);
        assert!(!task.is_done);
    }

    #[test]
    fn test_done() {
        let mut task = Task::new("text".to_string());
        assert!(!task.is_done);
        task.done();
        assert!(task.is_done);
    }

    #[test]
    fn test_undone() {
        let mut task = Task::new("text".to_string());
        assert!(!task.is_done);
        task.done();
        assert!(task.is_done);
        task.undone();
        assert!(!task.is_done);
    }

    #[test]
    fn test_alter() {
        let init_text = String::from("init");
        let next_text = String::from("next");

        let mut task = Task::new(init_text.clone());

        assert!(task.text == init_text);
        task.alter(next_text.clone());
        assert!(task.text == next_text);
    }
}
