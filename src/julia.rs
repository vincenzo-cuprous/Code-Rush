use std::fs;
use std::process::Command;

pub fn run_julia(file_name: &str) {
    if !file_name.ends_with(".jl") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .jl file.");
        return;
    }
    Command::new("julia")
        .arg(file_name)
        .status()
        .expect("Failed to run .jl file.");
}
