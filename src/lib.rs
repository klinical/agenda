pub mod data;
pub mod command;
pub mod config;
pub mod error;
pub mod task;
pub mod constants;

use std::{
    io::{BufRead,},
    {io, io::Write},
};

use std::str::FromStr;
use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use crate::command::{clear_screen, create_new_task, display_help, display_list, remove_task, update_task, Command};
use crate::data::Database;
use crate::error::{AgendaResult};

pub fn process_command(command: &Command, database: &mut Database) -> AgendaResult<()> {
    match command {
        Command::Help => display_help(),
        Command::List => display_list(database),
        Command::Add => create_new_task(database)?,
        Command::Remove => remove_task(database)?,
        Command::Mod => update_task(database)?,
        Command::Clear => clear_screen(),
        Command::Exit => std::process::exit(0),
    }
    Ok(())
}

pub fn prompt_input(prompt: &str) -> AgendaResult<String> {
    Ok(Input::<String>::with_theme(&constants::select_theme())
        .with_prompt(prompt)
        .interact_text()?)
}