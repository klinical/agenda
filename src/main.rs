
use std::fs::File;
use std::io::Read;
use std::{fs::{self}, path};
use agenda::{agenda::{Agenda}, F_DIR, F_PATH};

fn main() {
    display_welcome();
    let mut agenda = load_database().expect("Failed to open/create database. Ensure this application is run with all required permissions.");
    loop {
        if let Err(e) = agenda::process_input(&mut agenda) {
            println!("Failed processing input: {}", e);
        }
    }
}

fn display_welcome() {
    println!("** AGENDA - A simple todo app from the 80's!");
    println!("** You may list the available commands by running 'help'.\n");
}

fn load_database() -> Result<Agenda, agenda::error::AppError> {
    // Open the data file, read the contents, and deserialize it into the Agenda DB
    let path = path::Path::new(F_PATH);
    if path.exists() {
        let mut data = String::new();
        let _ = File::open(path)?.read_to_string(&mut data)?;
        Ok(Agenda::from(serde_json::from_str(&data)?))
    } else {
        if !path::Path::new(F_DIR).exists() {
            fs::create_dir_all(F_DIR)?;
        }
        File::create(path)?;
        Ok(Agenda::new())
    }
}