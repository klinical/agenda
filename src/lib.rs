mod agenda;
mod command;
mod config;
mod error;
mod task;

use crate::agenda::Agenda;
use crate::command::Command;

use error::AgendaResult;
use std::{
    fs::{File, OpenOptions},
    io::BufRead,
    {io, io::Write},
};

pub fn run(path: &str) {
    println!("** AGENDA - A simple todo app from the 80's!");
    println!("** You may list the available commands by running 'help'\n");

    let existed = std::path::Path::new(path).exists();

    let mut list = open_data_file(path);

    let mut agenda = if existed {
        Agenda::read_from_file(&mut list).unwrap()
    } else {
        let agenda = Agenda::new();
        let _ = list
            .write(serde_json::to_string_pretty(&agenda).unwrap().as_bytes())
            .unwrap();
        agenda
    };

    loop {
        if let Some(cmd) = prompt() {
            if command::process(cmd, &mut agenda).is_err() {
                println!("Failed processing command!\n");
            } else {
                let mut list = open_data_file(path);

                list.write(serde_json::to_string_pretty(&agenda).unwrap().as_bytes())
                    .expect("Failed to write updated Agenda to data file.");
            }
        } else {
            println!("Use the 'help' command to see a list of available commands.\n");
        }
    }
}

fn open_data_file(path: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open data file.")
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
