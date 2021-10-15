mod command;
mod task;

use crate::command::Command;

use std::{
    fs::File,
    io::BufRead,
    {io, io::Result, io::Write},
};

pub fn launch(list: &File) {
    welcome();

    loop {
        if let Some(cmd) = prompt() {
            if process(cmd, &list).is_err() {
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
    if let Some(Ok(input)) = input {
        command::command_from_input(&input)
    } else {
        None
    }
}

fn read_terminal_input() -> Option<Result<String>> {
    let _ = io::stdout().flush();
    let stdin = io::stdin();
    let x = stdin.lock().lines().next();

    x
}

pub fn process(cmd: Command, file: &File) -> Result<()> {
    match cmd {
        Command::Help => display_help(),
        Command::List => display_list(),
        Command::Add => create_new_task(file),
        Command::Remove => {}
        Command::Mod => {}
        Command::Clear => clear_screen()?,
        Command::Exit => std::process::exit(0),
        _ => {}
    }

    Ok(())
}

fn create_new_task(file: &File) {
    if let Some((name, desc)) = new_task_dialog() {
        // Create a task object
        let new_task = task::Task::from(name, desc);
        // Add task to the file
        // Display a message back to the user
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
        (Some(Ok(name)), Some(Ok(description))) => Some((name, description)),
        _ => None,
    }
}

fn display_help() {}

fn display_list() {}

fn clear_screen() -> Result<()> {
    Ok(())
}
