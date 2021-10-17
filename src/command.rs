use std::{fmt, str::FromStr};

use crate::{agenda, error, task};

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

pub fn process(cmd: Command, agenda: &mut agenda::Agenda) -> error::AgendaResult<()> {
    match cmd {
        Command::Help => display_help(),
        Command::List => display_list(agenda),
        Command::Add => create_new_task(agenda),
        Command::Remove => remove_task(agenda),
        Command::Mod => update_task(agenda),
        Command::Clear => clear_screen(),
        Command::Exit => std::process::exit(0),
    };

    Ok(())
}

pub fn create_new_task(agenda: &mut agenda::Agenda) {
    if let Some((name, desc, priority)) = new_task_dialog() {
        // Create a task object
        let new_task = task::Task::from(desc, priority);

        match new_task {
            Ok(task) => {
                // Add task to the file
                // Display a message back to the user
                println!("\nNew task\n  Name: {}\n  Description: {}\n  Priority: {}\nCreated sucessfully.\n", name, task.description(), task.priority());

                agenda.add_task(name, task);
            }
            Err(task_err) => println!("{}", task_err),
        }
    }
}

pub fn update_task(agenda: &mut agenda::Agenda) {
    print!("\nEnter task name to update: ");
    let target = crate::read_terminal_input();

    // Locate the task or quit if input is empty
    let target = match target {
        Some(target) => agenda.task(&target),
        None => {
            println!("Nothing to do.");
            return;
        }
    };

    let task = match target {
        Some(task) => task,
        None => {
            println!("No task found.");
            return;
        }
    };

    print!("\nEnter property name to update (desc, priority): ");
    let property = if let Some(property) = crate::read_terminal_input() {
        property
    } else {
        println!("Nothing to do.");
        return;
    };

    print!("\nEnter new value for {}: ", &property);
    let value = if let Some(value) = crate::read_terminal_input() {
        value
    } else {
        println!("Nothing to do.");
        return;
    };

    match property.to_lowercase().as_str() {
        "description" | "desc" => task.set_description(value),
        "priority" => {
            if let Err(e) = task.set_priority(value) {
                println!("{}", e);
                return;
            }
        }
        _ => println!("Invalid property!"),
    }

    println!("\nTask updated successfully.\n");
}

fn new_task_dialog() -> Option<(String, String, String)> {
    print!("Enter a taskname: ");
    let new_task_name = crate::read_terminal_input();
    print!("Enter a description: ");
    let new_task_description = crate::read_terminal_input();

    println!(
        "\nAvailable priorities (and their aliases) (not case-sensitive)
        Important and Urgent ('iu')
        Important and Not Urgent ('inu')
        Not Important and Urgent ('niu')
        Not Important and Not Urgent ('ninu')\n
        "
    );
    print!("Enter task priority, from one of the choices listed above: ");
    let new_task_priority = crate::read_terminal_input();

    match (new_task_name, new_task_description, new_task_priority) {
        (Some(name), Some(description), Some(priority)) => Some((name, description, priority)),
        _ => None,
    }
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

pub fn remove_task(agenda: &mut agenda::Agenda) {
    print!("Enter name of task to be deleted (THIS CANNOT BE UNDONE): ");
    if let Some(target) = crate::read_terminal_input() {
        if let Some((name, task)) = agenda.remove_task(&target) {
            println!(
                "Removed task: {} with description: {}.",
                name,
                task.description()
            )
        } else {
            println!("No task with name {} found.", target)
        }
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
