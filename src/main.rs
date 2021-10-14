use agenda;
use std::fs::{self, File, OpenOptions};

static F_DIR: &str = "./data/";
static F_PATH: &str = "./data/agenda.yaml";

fn main() {
    fs::create_dir(F_DIR).expect("Failed to initialize data directory");
    let mut list = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(F_PATH)
        .expect("Failed to open data file.");

    agenda::launch(&mut list);
    agenda::welcome();

    loop {
        if let Some(cmd) = agenda::prompt() {
            if agenda::process(cmd).is_err() {
                println!("Failed processing command!\n");
            }
        } else {
            println!("Use the 'help' command to see a list of available commands.\n");
        }
    }
}
