use std::fs;
use std::process::Command;

// Function to run Rust files
pub fn run_rust(file_name: &str) {
    if !file_name.ends_with(".rs") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .rs file.");
        return;
    }
    let executable = file_name.replace(".rs", "");
    Command::new("rustc")
        .arg(file_name)
        .status()
        .expect("Failed to compile .rs file.");
    Command::new(format!("./{}", executable))
        .status()
        .expect("Failed to execute .rs file.");
}

// Function to run Go files
pub fn run_golang(file_name: &str) {
    if !file_name.ends_with(".go") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .go file.");
        return;
    }
    Command::new("go")
        .arg("run")
        .arg(file_name)
        .status()
        .expect("Failed to run .go file.");
}

// Function to run Ruby files
pub fn run_rb(file_name: &str) {
    if !file_name.ends_with(".rb") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .rb file.");
        return;
    }
    Command::new("ruby")
        .arg(file_name)
        .status()
        .expect("Failed to run .rb file.");
}
