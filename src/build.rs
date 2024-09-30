use std::env;
use std::fs;
use std::process::Command;
use std::path::Path;

// Make the build_mingw function public
pub fn build_mingw(file_name: &str) {
    // Check if the file is a .c file and if it exists
    if !file_name.ends_with(".c") || !Path::new(file_name).exists() {
        eprintln!("Error: File does not exist or is not a .c file.");
        return;
    }

    // Create the output filename by replacing the .c extension with .exe
    let output_file = file_name.replace(".c", ".exe");

    // Use the MinGW compiler (x86_64-w64-mingw32-gcc) to compile the C file
    let status = Command::new("x86_64-w64-mingw32-gcc")
        .arg(file_name)
        .arg("-o")
        .arg(&output_file)
        .status()
        .expect("Failed to execute MinGW compiler.");

    // Check if the compilation was successful and provide appropriate feedback
    if status.success() {
        println!("Successfully compiled {} to {}", file_name, output_file);
    } else {
        eprintln!("Error: MinGW compilation failed.");
    }
}

// Function to build C files with GCC
fn build_c(file_name: &str) {
    if !file_name.ends_with(".c") || fs::metadata(file_name).is_err() {
        eprintln!("Error: File does not exist or is not a .c file.");
        return;
    }

    let status = Command::new("gcc")
        .arg(file_name)
        .arg("-o")
        .arg(file_name.trim_end_matches(".c"))
        .status()
        .expect("Failed to compile C file.");

    if !status.success() {
        eprintln!("Error: Compilation failed for C file.");
    } else {
        println!("Successfully compiled C file: {}", file_name);
    }
}

// Function to build C++ files
fn build_cpp(file_name: &str) {
    if !file_name.ends_with(".cpp") || fs::metadata(file_name).is_err() {
        eprintln!("Error: File does not exist or is not a .cpp file.");
        return;
    }

    let status = Command::new("g++")
        .arg(file_name)
        .arg("-o")
        .arg(file_name.trim_end_matches(".cpp"))
        .status()
        .expect("Failed to compile C++ file.");

    if !status.success() {
        eprintln!("Error: Compilation failed for C++ file.");
    } else {
        println!("Successfully compiled C++ file: {}", file_name);
    }
}

// Function to build C# files
fn build_cs(file_name: &str) {
    if !file_name.ends_with(".cs") || fs::metadata(file_name).is_err() {
        eprintln!("Error: File does not exist or is not a .cs file.");
        return;
    }

    // Assuming the use of 'dotnet' for building C# files
    let status = Command::new("dotnet")
        .arg("build")
        .arg(file_name)
        .status()
        .expect("Failed to compile C# file.");

    if !status.success() {
        eprintln!("Error: Compilation failed for C# file.");
    } else {
        println!("Successfully compiled C# file: {}", file_name);
    }
}

// Function to handle the build command
pub fn build_file(file_name: &str) {
    match file_name {
        f if f.ends_with(".c") && env::args().any(|arg| arg == "-w") => build_mingw(file_name), // Use MinGW if '-w' flag is provided
        f if f.ends_with(".c") => build_c(file_name),
        f if f.ends_with(".cpp") => build_cpp(file_name),
        f if f.ends_with(".cs") => build_cs(file_name), // Handle C# build
        _ => eprintln!("Error: Unsupported file type for building."),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo build <file_name> [-w]");
        return;
    }

    let file_name = &args[1];

    build_file(file_name);
}
