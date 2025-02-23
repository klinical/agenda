use dialoguer::theme::ColorfulTheme;

pub fn select_theme() -> ColorfulTheme {
    ColorfulTheme::default()
}

pub const F_DIR: &str = "./data/";
pub const F_PATH: &str = "./data/agenda.json";

pub const SELECT_PRIORITY_PROMPT: &str = "Select priority";
pub const SELECT_TASK_PROMPT: &str = "Select task id";

pub const TASK_NAME_PROMPT: &str = "Enter a task name: ";
pub const TASK_DESCRIPTION_PROMPT: &str = "Enter a description: ";
pub const ADD_ANOTHER_TASK_CONFIRMATION: &str = "Do you want to add another task?";

pub const REMOVE_TASK_CONFIRMATION: &str = "Are you sure you wish to delete this task? (This cannot be undone)";
pub const REMOVE_ANOTHER_TASK_CONFIRMATION: &str = "Do you want to remove another task?";