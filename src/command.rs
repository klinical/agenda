use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    List,
    Add,
    Remove,
    Mod,
    Clear,
    Exit,
}

#[derive(Debug, Clone)]
pub struct CommandError;

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid/unrecognized command.")
    }
}

// This and command_from_input should be one method
impl FromStr for Command {
    type Err = CommandError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "help" | "h" => Ok(Command::Help),
            "list" | "ls" => Ok(Command::List),
            "add" | "a" => Ok(Command::Add),
            "remove" | "rm" => Ok(Command::Remove),
            "modify" | "mod" => Ok(Command::Mod),
            "clear" | "x" => Ok(Command::Clear),
            "quit" | "q" => Ok(Command::Exit),
            _ => Err(CommandError),
        }
    }
}

pub fn command_from_input(input: &str) -> Option<Command> {
    // But was it whitespace/empty?
    if input == "\n" || input.is_empty() || input == " " {
        None
    } else {
        // Ok - it wasn't... does it match to a valid command?
        match Command::from_str(input) {
            Ok(cmd) => Some(cmd),
            Err(cmd_err) => {
                println!("{}", cmd_err);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::*;

    #[test]
    fn add_from_input() {
        assert!(command_from_input("add") == Some(Command::Add));
    }

    #[test]
    fn no_command_from_input() {
        assert!(command_from_input("") == None);
    }

    #[test]
    fn invalid_from_input() {
        assert!(command_from_input("DNE") == None);
    }
}
