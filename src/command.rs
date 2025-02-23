use std::{fmt};
use std::fmt::Formatter;
use dialoguer::{Confirm, Select};
use crate::{data, constants};
use crate::error::{AgendaResult, AppError};
use crate::task::{Priority, Task};

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

impl fmt::Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Command::Help => write!(f, "View Help"),
            Command::List => write!(f, "List Tasks"),
            Command::Add => write!(f, "Add Task"),
            Command::Remove => write!(f, "Remove Task"),
            Command::Mod => write!(f, "Modify Task"),
            Command::Clear => write!(f, "Clear Terminal"),
            Command::Exit => write!(f, "Quit Program"),
        }
    }
}

pub fn create_new_task(agenda: &mut data::Database) -> AgendaResult<()> {
    loop {
        let name = crate::prompt_input(constants::TASK_NAME_PROMPT)?;
        let description = crate::prompt_input(constants::TASK_DESCRIPTION_PROMPT)?;
        let priority = select_priority()?;
        agenda.add_task(Task::new(name, description, priority))?;
        if !Confirm::new().with_prompt(constants::ADD_ANOTHER_TASK_CONFIRMATION).interact()? {
            break;
        }
    }

    Ok(())
}

pub fn select_priority() -> AgendaResult<Priority> {
    let priorities = Priority::values();
    let priority_selection = Select::with_theme(&constants::select_theme())
        .with_prompt(constants::SELECT_PRIORITY_PROMPT)
        .items(priorities)
        .default(0)
        .interact()?;
    priorities.get(priority_selection).copied().ok_or_else(|| AppError::InputError("Error selecting priority".to_string()))
}

pub fn update_task(agenda: &mut data::Database) -> Result<(), AppError> {
    // let target = agenda.task(&crate::prompt_input("\nEnter task name to update: ")?);
    // let mut task = match target {
    //     Some(task) => task,
    //     None => {
    //         println!("No task found.");
    //         return Ok(());
    //     }
    // };
    //
    // let property = crate::prompt_input("\nEnter property name to update (desc, priority): ")?;
    // let value = crate::prompt_input(&format!("\nEnter new value for {}: ", &property))?;
    //
    // match property.to_lowercase().as_str() {
    //     "description" | "desc" => task.set_description(value),
    //     "priority" => {
    //         if let Err(e) = task.set_priority(value) {
    //             println!("{}", e);
    //             return Ok(());
    //         }
    //     }
    //     _ => println!("Invalid property!"),
    // }
    //
    // println!("\nTask updated successfully.\n");
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

pub fn remove_task(agenda: &mut data::Database) -> Result<(), AppError> {
    loop {
        let task_list = agenda.tasks().iter().enumerate().map(|(idx, task)| {
            format!("{}. {}", idx, task.name())
        }).collect::<Vec<String>>();
        let task_selection = Select::with_theme(&constants::select_theme())
            .with_prompt(constants::SELECT_TASK_PROMPT)
            .items(&task_list)
            .default(0)
            .interact_opt()?;

        // Some(i) if ... chains. The first branch handles the case of there being Some(idx) and the user confirms yes to remove task.
        // then, it removes the task and does another confirmation whether to delete another or not (and break if so).
        // The second Some handles the case the user picks an index, but decides not to remove the task and continues the loop (thus, the idx is ignored via _).
        // Finally, None handles no index being picked, and breaks the loop.
        match task_selection {
            Some(index) if Confirm::new().with_prompt(constants::REMOVE_TASK_CONFIRMATION).interact()? => {
                agenda.remove_task(index)?;
                if !Confirm::new().with_prompt(constants::REMOVE_ANOTHER_TASK_CONFIRMATION).interact()? {
                    break;
                }
            },
            Some(_) => continue,
            None => break,
        }
    }

    Ok(())
}

pub fn display_list(agenda: &data::Database) {
    println!("Available tasks:");
    for (idx, task) in agenda.tasks().iter().enumerate() {
        println!(
            "\n{}: {}\n {}\n  Priority: {}\n",
            idx,
            task.name(),
            task.description(),
            task.priority(),
        )
    }
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
