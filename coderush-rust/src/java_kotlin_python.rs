use std::fs;
use std::process::Command;

// Function to run Java files
pub fn run_java(file_name: &str) {
    if !file_name.ends_with(".java") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .java file.");
        return;
    }
    let base_name = file_name.replace(".java", "");
    Command::new("javac")
        .arg(file_name)
        .status()
        .expect("Failed to compile .java file.");
    Command::new("java")
        .arg(base_name)
        .status()
        .expect("Failed to run .class file.");
}

// Function to run Kotlin files
pub fn run_kotlin(file_name: &str) {
    if !file_name.ends_with(".kt") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .kt file.");
        return;
    }
    let jar_file = file_name.replace(".kt", ".jar");
    let compile_result = Command::new("kotlinc")
        .arg(file_name)
        .arg("-include-runtime")
        .arg("-d")
        .arg(&jar_file)
        .output()
        .expect("Failed to compile .kt file.");
    if !compile_result.status.success() {
        println!(
            "Compilation failed:\n{}",
            String::from_utf8_lossy(&compile_result.stderr)
        );
        return;
    }
    let run_result = Command::new("kotlin")
        .arg(jar_file)
        .output()
        .expect("Failed to run .jar file.");
    if !run_result.status.success() {
        println!(
            "Execution failed:\n{}",
            String::from_utf8_lossy(&run_result.stderr)
        );
    } else {
        println!("Program output:\n{}", String::from_utf8_lossy(&run_result.stdout));
    }
}

// Function to run Python files
pub fn run_py(file_name: &str) {
    if !file_name.ends_with(".py") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .py file.");
        return;
    }
    Command::new("python3")
        .arg(file_name)
        .status()
        .expect("Failed to run .py file.");
}
