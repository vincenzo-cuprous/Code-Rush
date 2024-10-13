use std::fs;
use std::process::Command;

pub fn run_c(file_name: &str) {
    if !file_name.ends_with(".c") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .c file.");
        return;
    }

    let executable = file_name.replace(".c", ".out");
    let compile_status = Command::new("gcc")
        .arg(file_name)
        .arg("-o")
        .arg(&executable)
        .status()
        .expect("Failed to compile .c file.");

    if !compile_status.success() {
        eprintln!("Error: Compilation of '{}' failed.", file_name);
        return;
    }

    let run_status = Command::new(format!("./{}", executable))
        .status()
        .expect("Failed to execute .out file.");

    if !run_status.success() {
        eprintln!("Error: Execution of '{}' failed.", executable);
    }
}

pub fn run_cpp(file_name: &str) {
    if !file_name.ends_with(".cpp") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .cpp file.");
        return;
    }

    let executable = file_name.replace(".cpp", ".out");
    let compile_status = Command::new("g++")
        .arg(file_name)
        .arg("-o")
        .arg(&executable)
        .status()
        .expect("Failed to compile .cpp file.");

    if !compile_status.success() {
        eprintln!("Error: Compilation of '{}' failed.", file_name);
        return;
    }

    let run_status = Command::new(format!("./{}", executable))
        .status()
        .expect("Failed to execute .out file.");

    if !run_status.success() {
        eprintln!("Error: Execution of '{}' failed.", executable);
    }
}

pub fn run_cs(file_name: &str) {
    if !file_name.ends_with(".cs") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .cs file.");
        return;
    }

    let exe_name = file_name.replace(".cs", ".exe");
    let compile_status = Command::new("mcs")
        .arg(file_name)
        .status()
        .expect("Failed to compile .cs file.");

    if !compile_status.success() {
        eprintln!("Error: Compilation of '{}' failed.", file_name);
        return;
    }

    if fs::metadata(&exe_name).is_ok() {
        let run_status = Command::new("mono")
            .arg(&exe_name)
            .status()
            .expect("Failed to execute .exe file.");

        if !run_status.success() {
            eprintln!("Error: Execution of '{}' failed.", exe_name);
        }
    } else {
        println!("Compilation failed: '{}'", exe_name);
    }
}

pub fn run_dot(file_name: &str) {
    if !file_name.ends_with(".cs") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .cs file.");
        return;
    }

    let run_status = Command::new("dotnet")
        .arg("run")
        .arg(file_name)
        .status()
        .expect("Failed to run .cs file.");

    if !run_status.success() {
        eprintln!("Error: Execution of '{}' failed.", file_name);
    }
}
