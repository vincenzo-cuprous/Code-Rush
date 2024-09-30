// build.rs
use std::fs;
use std::process::Command;

// Function to build C files
fn build_c(file_name: &str) {
    if !file_name.ends_with(".c") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .c file.");
        return;
    }
    Command::new("gcc")
        .arg(file_name)
        .arg("-o")
        .arg(file_name.trim_end_matches(".c"))
        .status()
        .expect("Failed to compile C file.");
}

// Function to build C++ files
fn build_cpp(file_name: &str) {
    if !file_name.ends_with(".cpp") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .cpp file.");
        return;
    }
    Command::new("g++")
        .arg(file_name)
        .arg("-o")
        .arg(file_name.trim_end_matches(".cpp"))
        .status()
        .expect("Failed to compile C++ file.");
}

// Function to build Rust files
fn build_rust(file_name: &str) {
    if !file_name.ends_with(".rs") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .rs file.");
        return;
    }
    Command::new("rustc")
        .arg(file_name)
        .status()
        .expect("Failed to compile Rust file.");
}

// Function to build Java files
fn build_java(file_name: &str) {
    if !file_name.ends_with(".java") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .java file.");
        return;
    }
    Command::new("javac")
        .arg(file_name)
        .status()
        .expect("Failed to compile Java file.");
}

// Add more functions for other languages...

// General build function that detects the file type and calls the appropriate build function
pub fn build_file(file_name: &str) {
    match file_name {
        f if f.ends_with(".c") => build_c(f),
        f if f.ends_with(".cpp") => build_cpp(f),
        f if f.ends_with(".rs") => build_rust(f),
        f if f.ends_with(".java") => build_java(f),
        // Add more languages here...
        _ => eprintln!("Error: Unsupported file type for building."),
    }
}
