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
    welcome();

    let existed = std::path::Path::new(path).exists();

    let mut list = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open data file.");

    let agenda = if existed {
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
            if process(cmd, &mut list).is_err() {
                println!("Failed processing command!\n");
            } else {
            }
        } else {
            println!("Use the 'help' command to see a list of available commands.\n");
        }
    }
}

pub fn welcome() {
    println!("** AGENDA - A simple todo app from the 80's!");
    println!("** You may list the available commands by running 'help'\n")
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

pub fn process(cmd: Command, file: &mut File) -> AgendaResult<()> {
    match cmd {
        Command::Help => display_help(),
        Command::List => display_list(),
        Command::Add => create_new_task(file),
        Command::Remove => {}
        Command::Mod => {}
        Command::Clear => clear_screen()?,
        Command::Exit => std::process::exit(0),
    };

    Ok(())
}

fn create_new_task(file: &mut File) {
    if let Some((name, desc)) = new_task_dialog() {
        // Create a task object
        let new_task = task::Task::from(name, desc);
        // Add task to the file
        // Display a message back to the user
        let task_json = serde_json::to_string_pretty(&new_task).unwrap();
        file.write(task_json.as_bytes());
        println!(
            "New task '{}': '{}' created.",
            new_task.name(),
            new_task.description()
        );
    }
}

fn new_task_dialog() -> Option<(String, String)> {
    print!("Enter a taskname: ");
    let new_task_name = read_terminal_input();
    print!("Enter a description: ");
    let new_task_description = read_terminal_input();

    match (new_task_name, new_task_description) {
        (Some(name), Some(description)) => Some((name, description)),
        _ => None,
    }
}

fn display_help() {}

fn display_list() {}

fn clear_screen() -> AgendaResult<()> {
    Ok(())
}
