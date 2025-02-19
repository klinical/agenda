use std::{
    fs::{self},
    io::ErrorKind,
};

fn main() {
    if let Err(e) = fs::create_dir(agenda::F_DIR) {
        if e.kind() != ErrorKind::AlreadyExists {
            println!("Failed to open/create data directory. Ensure this application is run with all required permissions.");
        }
    }

    agenda::run().expect("TODO: panic message");
}
