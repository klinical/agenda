mod command;

use command::Command;

use std::{
    fs::File,
    io::BufRead,
    str::FromStr,
    {io, io::Write},
};

pub fn launch(list: &File) {}

pub fn welcome() {
    println!("** AGENDA - A simple todo app from the 80's!\n\n");
}

pub fn prompt() -> Option<Command> {
    print!("input a command ('help' for help): ");

    let _ = io::stdout().flush();
    let stdin = io::stdin();
    let input = stdin.lock().lines().next();

    // Got Some input that was Ok
    if let Some(Ok(input)) = input {
        // But was it whitespace/empty?
        if input == "\n" || input == "" || input == " " {
            None
        } else {
            // Ok - it wasn't... does it match to a valid command?
            match Command::from_str(&input) {
                Ok(cmd) => Some(cmd),
                Err(cmd_err) => {
                    println!("{}", cmd_err);
                    None
                }
            }
        }
    } else {
        None
    }
}

pub fn process(cmd: Command) -> Result<(), std::io::Error> {
    match cmd {
        Command::Help => display_help(),
        Command::List => display_list(),
        _ => {}
    }

    Ok(())
}

fn display_help() {}

fn display_list() {}
