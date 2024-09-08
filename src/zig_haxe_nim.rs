use std::fs;
use std::process::Command;

pub fn run_zig(file_name: &str) {
    if !file_name.ends_with(".zig") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .zig file.");
        return;
    }
    Command::new("zig")
        .arg("run")
        .arg(file_name)
        .status()
        .expect("Failed to run .zig file.");
}

pub fn run_haxe(file_name: &str) {
    if !file_name.ends_with(".hx") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .hx file.");
        return;
    }
    Command::new("haxe")
        .arg("-main")
        .arg(file_name)
        .arg("--interp")
        .status()
        .expect("Failed to interpret .hx file.");
}

pub fn run_nim(file_name: &str) {
    if !file_name.ends_with(".nim") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .nim file.");
        return;
    }
    Command::new("nim")
        .arg("c")
        .arg("-r")
        .arg("--verbosity:0")
        .arg(file_name)
        .status()
        .expect("Failed to compile and run .nim file.");
}
