use agenda::{data::{Database}, F_PATH};

fn main() {
    display_welcome();
    let mut database = Database::open(F_PATH.as_ref()).expect("Failed to open/create database. Ensure this application is run with all required permissions.");
    loop {
        if let Err(e) = agenda::process_input(&mut database) {
            println!("Failed processing input: {}", e);
        }
    }
}

fn display_welcome() {
    println!("** AGENDA - A simple todo app from the 80's!");
    println!("** You may list the available commands by running 'help'.\n");
}