use std::fs;
use std::process::Command;

// Function to run Mojo files
pub fn run_mojo(file_name: &str) {
    if !file_name.ends_with(".mojo") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .mojo file.");
        return;
    }
    // Replace `mojo` with the actual command to run Mojo files if different
    Command::new("mojo")
        .arg(file_name)
        .status()
        .expect("Failed to run .mojo file.");
}
