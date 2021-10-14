use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    List,
    Add,
    Remove,
    Mod,
}

#[derive(Debug, Clone)]
pub struct CommandError;

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid/unrecognized command.")
    }
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "help" | "h" => Ok(Command::Help),
            "list" | "ls" => Ok(Command::List),
            "add" | "a" => Ok(Command::Add),
            "remove" | "rm" => Ok(Command::Remove),
            "modify" | "mod" => Ok(Command::Mod),
            _ => Err(CommandError),
        }
    }
}
