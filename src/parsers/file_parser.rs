use crate::models::task::Task;

pub struct FileParserResult {
    tasks: Vec<Task>,
}

pub fn parse(_text: Vec<&str>) -> FileParserResult {
    // TODO: Implementation
    FileParserResult { tasks: vec![] }
}
