pub mod data;
pub mod command;
pub mod config;
pub mod error;
pub mod task;

use std::{
    fs::File,
    io::{BufRead, ErrorKind},
    {io, io::Write},
};
use std::path;
use std::str::FromStr;

pub static F_DIR: &str = "./data/";
pub static F_PATH: &str = "./data/agenda.json";

pub fn process_input(agenda: &mut data::Database) -> Result<(), error::AppError>{
    // First, if there is any errors during input catch it. Then check if the command is valid
    let user_input = prompt_input("input a command ('help' for help): ")?;
    if let Ok(cmd) = command::Command::from_str(&user_input) {
        if command::Command::process(cmd, agenda).is_err() {
            println!("Failed processing command!\n");
        } else {
            open_data_file(F_PATH.as_ref(), "w").write_all(serde_json::to_string_pretty(&agenda)?.as_bytes())?;
        }
    } else {
        println!("Use the 'help' command to see a list of available commands.\n");
    }
    Ok(())
}

pub fn open_data_file(path: &path::Path, mode: &str) -> File {
    let f = match mode {
        "r" => File::open(path),
        "w" => File::create(path),
        _ => panic!("Invalid file opening mode"),
    };

    match f {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => File::create(path)
                .expect("Unable to create data file. Ensure you have the correct permissions."),
            _ => panic!("Unable to open data file. Ensure you have the correct permissions."),
        },
    }
}

pub fn prompt_input(prompt: &str) -> Result<String, error::AppError> {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    match io::stdin().lock().lines().next() {
        Some(Ok(line)) => Ok(line),
        _ => Err(error::AppError::InputError("Failed to read input.".to_string())),
    }
}