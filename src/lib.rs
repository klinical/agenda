mod agenda;
mod command;
mod config;
mod error;
mod task;

use crate::agenda::Agenda;
use crate::command::Command;
use std::{
    fs::File,
    io::{BufRead, ErrorKind},
    {io, io::Write},
};

pub static F_DIR: &str = "./data/";
static F_PATH: &str = "./data/agenda.json";

pub fn run() {
    println!("** AGENDA - A simple todo app from the 80's!");
    println!("** You may list the available commands by running 'help'\n");

    let existed = std::path::Path::new(F_PATH).exists();

    let mut list = open_data_file(F_PATH, "r");

    let mut agenda = if existed {
        Agenda::read_from_file(&mut list).unwrap()
    } else {
        let agenda = Agenda::new();
        let _ = list
            .write_all(serde_json::to_string_pretty(&agenda).unwrap().as_bytes())
            .unwrap();
        agenda
    };

    // Ensure the file descriptor is freed before we move forward
    std::mem::drop(list);

    loop {
        if let Some(cmd) = prompt() {
            if command::process(cmd, &mut agenda).is_err() {
                println!("Failed processing command!\n");
            } else {
                let mut file = open_data_file(F_PATH, "w");

                file.write_all(serde_json::to_string_pretty(&agenda).unwrap().as_bytes())
                    .expect("Failed to write updated Agenda to data file.");
            }
        } else {
            println!("Use the 'help' command to see a list of available commands.\n");
        }
    }
}

fn open_data_file(path: &str, mode: &str) -> File {
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

pub fn prompt() -> Option<Command> {
    print!("input a command ('help' for help): ");
    let input = read_terminal_input();

    // Got Some input that was Ok
    if let Some(input) = input {
        command::command_from_input(&input)
    } else {
        None
    }
}

fn read_terminal_input() -> Option<String> {
    let _ = io::stdout().flush();
    let stdin = io::stdin();
    let x = stdin.lock().lines().next();

    match x {
        Some(Ok(x)) => Some(x),
        _ => None,
    }
}
