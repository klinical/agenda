use agenda;
use std::{
    fs::{self, File, OpenOptions},
    io::ErrorKind,
};

static F_DIR: &str = "./data/";
static F_PATH: &str = "./data/agenda.yaml";

fn main() {
    if let Err(e) = fs::create_dir(F_DIR) {
        if e.kind() != ErrorKind::AlreadyExists {
            println!("Failed to open/create data directory. Ensure this application is run with all required permissions.");
        }
    }

    let mut list = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(F_PATH)
        .expect("Failed to open data file.");

    agenda::launch(&mut list);
}
