use std::{fmt};
use std::fmt::Formatter;
use dialoguer::{Confirm, Select};
use crate::{data, constants};
use crate::data::Database;
use crate::error::{AgendaResult, AppError};
use crate::task::{Priority, Property, Task};

#[derive(Debug, PartialEq)]
pub enum Command {
    List,
    Add,
    Remove,
    Mod,
    Exit,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Command::List => write!(f, "List Tasks"),
            Command::Add => write!(f, "Add Task"),
            Command::Remove => write!(f, "Remove Task"),
            Command::Mod => write!(f, "Modify Task"),
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

pub fn select_task(database: &Database) -> AgendaResult<Option<usize>> {
    let task_list = database.tasks().iter().enumerate().map(|(idx, task)| {
        format!("{}. {}", idx, task.name())
    }).collect::<Vec<String>>();
    Ok(Select::with_theme(&constants::select_theme())
        .with_prompt(constants::SELECT_TASK_PROMPT)
        .items(&task_list)
        .default(0)
        .interact_opt()?)
}

pub fn select_property() -> AgendaResult<Option<Property>> {
    let properties = Property::values();
    let property_selection = Select::with_theme(&constants::select_theme())
        .with_prompt(constants::SELECT_PROPERTY_PROMPT)
        .items(Property::values())
        .default(0)
        .interact_opt()?;
    match property_selection {
        Some(idx) => {
            let property = properties.get(idx).cloned().ok_or_else(|| AppError::InputError("Error selecting property".to_string()))?;
            Ok(Some(property))
        }
        None => Ok(None)
    }
}

pub fn update_task(database: &mut Database) -> Result<(), AppError> {
    if database.tasks().is_empty() {
        return Ok(println!("No tasks to update."));
    }

    while let Some(task_id) = select_task(database)? {
        if let Some(property) = select_property()? {
            match property {
                Property::Name => {
                    let new_name = crate::prompt_input(constants::UPDATE_TASK_NAME_PROMPT)?;
                    database.update_task_name(task_id, new_name)?;
                }
                Property::Description => {
                    let new_description = crate::prompt_input(constants::UPDATE_TASK_DESCRIPTION_PROMPT)?;
                    database.update_task_name(task_id, new_description)?;
                }
                Property::Priority => {
                    let new_priority = select_priority()?;
                    println!("New priority: {}", new_priority);
                    database.update_task_priority(task_id, new_priority)?;
                }
            }

            if !Confirm::new().with_prompt(constants::UPDATE_ANOTHER_TASK_CONFIRMATION).interact()? {
                break;
            }
        } else {
            break;
        }
    }
    Ok(())
}

pub fn remove_task(database: &mut Database) -> Result<(), AppError> {
    if database.tasks().is_empty() {
        return Ok(println!("No tasks to remove."));
    }

    loop {
        let task_selection = select_task(database)?;

        // Some(i) if ... chains. The first branch handles the case of there being Some(idx) and the user confirms yes to remove task.
        // then, it removes the task and does another confirmation whether to delete another or not (and break if so).
        // The second Some handles the case the user picks an index, but decides not to remove the task and continues the loop (thus, the idx is ignored via _).
        // Finally, None handles no index being picked, and breaks the loop.
        match task_selection {
            Some(index) if Confirm::new().with_prompt(constants::REMOVE_TASK_CONFIRMATION).interact()? => {
                database.remove_task(index)?;
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

pub fn display_list(database: &Database) {
    if database.tasks().is_empty() {
        return println!("No tasks to update.");
    }

    println!("Available tasks:");
    for (idx, task) in database.tasks().iter().enumerate() {
        println!(
            "\n{}: {}\n {}\n  Priority: {}\n",
            idx,
            task.name(),
            task.description(),
            task.priority(),
        )
    }
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
