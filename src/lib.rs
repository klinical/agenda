pub mod data;
pub mod command;
pub mod config;
pub mod error;
pub mod task;
pub mod constants;


use dialoguer::Input;
use crate::command::{create_new_task,  display_list, remove_task, update_task, Command};
use crate::data::Database;
use crate::error::{AgendaResult};

pub fn process_command(command: &Command, database: &mut Database) -> AgendaResult<()> {
    match command {
        Command::List => display_list(database),
        Command::Add => create_new_task(database)?,
        Command::Remove => remove_task(database)?,
        Command::Mod => update_task(database)?,
        Command::Exit => std::process::exit(0),
    }
    Ok(())
}

pub fn prompt_input(prompt: &str) -> AgendaResult<String> {
    Ok(Input::<String>::with_theme(&constants::select_theme())
        .with_prompt(prompt)
        .interact_text()?)
}