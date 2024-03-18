pub enum ProcessError {
    ListCapacityExceeded,
    ListItemNotFound,
    TaskIndexMissing,
    InvalidArguments,
    UnknownCommand,
    CannotCreateFile,
    CannotWriteToFile,
}

pub enum ProcessResult {
    Sh,
    Ok,
    Terminate,
    Feedback(String),
    Error(ProcessError),
}

pub const TASKS_LIST_MAX_CAPACITY: usize = 200;

pub const C_EXIT: &str = "exit";
pub const C_HELP: &str = "help";
pub const C_LIST: &str = "list";
pub const C_ADD: &str = "add";
pub const C_REMOVE: &str = "remove";
pub const C_EDIT: &str = "edit";
pub const C_DONE: &str = "done";
pub const C_UNDONE: &str = "undone";
pub const C_CLEAR: &str = "clear";
pub const C_SAVE: &str = "save";
pub const C_LOAD: &str = "LOAD";

pub const C_Y: &str = "y";
pub const C_YES: &str = "yes";
