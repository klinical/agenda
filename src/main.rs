use dialoguer::Select;
use agenda::{constants, data::{Database}, process_command};
use agenda::command::Command;


fn main() {
    let mut database = Database::open(constants::F_PATH.as_ref()).expect("Failed to open/create database. Ensure this application is run with all required permissions.");
    println!("** AGENDA - A simple todo app from the 90's! **");
    let actions = vec![Command::Add, Command::Mod, Command::Remove, Command::List, Command::Exit];
    loop {
        let selection = Select::with_theme(&constants::select_theme())
            .with_prompt("Choose an action")
            .items(&actions)
            .default(0)
            .interact()
            .expect("Failed to select an action.");
        let command = actions.get(selection).unwrap_or_else(|| {
            println!("Invalid selection.");
            &Command::Exit
        });
        process_command(command, &mut database).unwrap_or_else(|err| println!("Could not perform command '{}': {}", command, err));
    }
}