use crate::models::task::Task;

pub struct FileParserResult<'a> {
    tasks: Vec<Task<'a>>,
}

pub fn parse(_text: Vec<&str>) -> FileParserResult {
    // TODO: Implementation
    FileParserResult { tasks: vec![] }
}
