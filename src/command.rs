use std::{fmt, str::FromStr};

use crate::{agenda, task};
use crate::error::AppError;

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

impl Command {
    pub fn process(cmd: Command, agenda: &mut agenda::Agenda) -> Result<(), AppError> {
        match cmd {
            Command::Help => display_help(),
            Command::List => display_list(agenda),
            Command::Add => create_new_task(agenda)?,
            Command::Remove => remove_task(agenda)?,
            Command::Mod => update_task(agenda)?,
            Command::Clear => clear_screen(),
            Command::Exit => std::process::exit(0),
        };

        Ok(())
    }
}

impl FromStr for Command {
    type Err = AppError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "help" | "h" => Ok(Command::Help),
            "list" | "ls" => Ok(Command::List),
            "add" | "a" => Ok(Command::Add),
            "remove" | "rm" => Ok(Command::Remove),
            "modify" | "mod" => Ok(Command::Mod),
            "clear" | "x" => Ok(Command::Clear),
            "quit" | "q" => Ok(Command::Exit),
            _ => Err(AppError::InputError("Invalid/unrecognized command.".to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandError;

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid/unrecognized command.")
    }
}

pub fn create_new_task(agenda: &mut agenda::Agenda) -> Result<(), AppError> {
    let name = crate::prompt_input("Enter a task name: ")?;
    let description = crate::prompt_input("Enter a description: ")?;
    let priority = crate::prompt_input(
        "\nAvailable priorities (and their aliases) (not case-sensitive)
        Important and Urgent ('iu')
        Important and Not Urgent ('inu')
        Not Important and Urgent ('niu')
        Not Important and Not Urgent ('ninu')\n\nEnter task priority, from one of the choices listed above:")?;

    // Need to just set name here.
    let new_task = task::Task::from(description, priority);
    // TODO! Problem here is - what if the Priority fails to parse? Instead of sending user all the way back, we should
    // give the user another chance to select a valid priority, or, let them cancel task creation entirely.
    match new_task {
        Ok(task) => {
            println!("\nNew task\n  Name: {}\n  Description: {}\n  Priority: {}\nCreated sucessfully.\n", name, task.description(), task.priority());
            Ok(agenda.add_task(name, task))
        }
        Err(priority_err) => Err(AppError::InputError(priority_err.to_string())),
    }
}

pub fn update_task(agenda: &mut agenda::Agenda) -> Result<(), AppError> {
    let target = agenda.task(&crate::prompt_input("\nEnter task name to update: ")?);
    let task = match target {
        Some(task) => task,
        None => {
            println!("No task found.");
            return Ok(());
        }
    };

    let property = crate::prompt_input("\nEnter property name to update (desc, priority): ")?;
    let value = crate::prompt_input(&format!("\nEnter new value for {}: ", &property))?;

    match property.to_lowercase().as_str() {
        "description" | "desc" => task.set_description(value),
        "priority" => {
            if let Err(e) = task.set_priority(value) {
                println!("{}", e);
                return Ok(());
            }
        }
        _ => println!("Invalid property!"),
    }

    println!("\nTask updated successfully.\n");
    Ok(())
}

pub fn display_help() {
    println!(
        "\n** Available commands (and their aliases) **\n
    'help'   ('h'):   Displays this menu
    'list'   ('ls'):  List all current tasks
    'add'    ('a'):   Open the new task creation dialog
    'remove' ('rm'):  Open the remove task dialog
    'modify' ('mod'): Open the task modification dialog
    'clear'  ('x'):   Clear/flush the terminal screen
    'quit'   ('q'):   Quit the program\n"
    )
}

pub fn remove_task(agenda: &mut agenda::Agenda) -> Result<(), AppError> {
    let target = crate::prompt_input("Enter name of task to be deleted (THIS CANNOT BE UNDONE): ")?;
    if let Some((name, task)) = agenda.remove_task(&target) {
        println!(
            "Removed task: {} with description: {}.",
            name,
            task.description()
        );
        Ok(())
    } else {
        println!("No task with name {} found.", target);
        Err(AppError::InputError("No task with that name found.".to_string()))
    }
}

pub fn display_list(agenda: &agenda::Agenda) {
    agenda.tasks_iter().for_each(|(name, task)| {
        println!(
            "\nTask Name: {}\n  Description: {}\n  Priority: {}\n",
            name,
            task.description(),
            task.priority(),
        )
    })
}

// Tested on Manjaro Linux
pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char)
}

#[cfg(test)]
mod tests {
    // use crate::command::*;

    // #[test]
    // fn add_from_input() {
    //     assert!(command_from_input("add") == Some(Command::Add));
    // }
    //
    // #[test]
    // fn no_command_from_input() {
    //     assert!(command_from_input("") == None);
    // }
    //
    // #[test]
    // fn invalid_from_input() {
    //     assert!(command_from_input("DNE") == None);
    // }
}
