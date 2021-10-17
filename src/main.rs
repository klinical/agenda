use std::{
    fs::{self},
    io::ErrorKind,
};

static F_DIR: &str = "./data/";
static F_PATH: &str = "./data/agenda.json";

fn main() {
    if let Err(e) = fs::create_dir(F_DIR) {
        if e.kind() != ErrorKind::AlreadyExists {
            println!("Failed to open/create data directory. Ensure this application is run with all required permissions.");
        }
    }

    agenda::run(F_PATH);
}
