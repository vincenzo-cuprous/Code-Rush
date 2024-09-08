use std::fs;
use std::process::Command;

pub fn run_c(file_name: &str) {
    if !file_name.ends_with(".c") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .c file.");
        return;
    }
    let executable = file_name.replace(".c", ".out");
    Command::new("gcc")
        .arg(file_name)
        .arg("-o")
        .arg(&executable)
        .status()
        .expect("Failed to compile .c file.");
    Command::new(format!("./{}", executable))
        .status()
        .expect("Failed to execute .out file.");
}

pub fn run_cpp(file_name: &str) {
    if !file_name.ends_with(".cpp") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .cpp file.");
        return;
    }
    let executable = file_name.replace(".cpp", ".out");
    Command::new("g++")
        .arg(file_name)
        .arg("-o")
        .arg(&executable)
        .status()
        .expect("Failed to compile .cpp file.");
    Command::new(format!("./{}", executable))
        .status()
        .expect("Failed to execute .out file.");
}

pub fn run_cs(file_name: &str) {
    if !file_name.ends_with(".cs") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .cs file.");
        return;
    }
    let exe_name = file_name.replace(".cs", ".exe");
    Command::new("mcs")
        .arg(file_name)
        .status()
        .expect("Failed to compile .cs file.");
    if fs::metadata(&exe_name).is_ok() {
        Command::new("mono")
            .arg(&exe_name)
            .status()
            .expect("Failed to execute .exe file.");
    } else {
        println!("Compilation failed.");
    }
}

pub fn run_dot(file_name: &str) {
    if !file_name.ends_with(".cs") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .cs file.");
        return;
    }
    Command::new("dotnet")
        .arg("run")
        .arg(file_name)
        .status()
        .expect("Failed to run .cs file.");
}
