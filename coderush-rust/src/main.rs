use std::env;
use std::process::Command;
use std::fs;

fn run_c(file_name: &str) {
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

fn run_cpp(file_name: &str) {
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

fn run_cs(file_name: &str) {
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

fn run_rb(file_name: &str) {
    if !file_name.ends_with(".rb") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .rb file.");
        return;
    }
    Command::new("ruby")
        .arg(file_name)
        .status()
        .expect("Failed to run .rb file.");
}

fn run_hx(file_name: &str) {
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

fn run_dot(file_name: &str) {
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

fn run_golang(file_name: &str) {
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

fn run_jul(file_name: &str) {
    if !file_name.ends_with(".jl") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .jl file.");
        return;
    }
    Command::new("julia")
        .arg(file_name)
        .status()
        .expect("Failed to run .jl file.");
}

fn run_java(file_name: &str) {
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

fn run_py(file_name: &str) {
    if !file_name.ends_with(".py") || !fs::metadata(file_name).is_ok() {
        println!("Error: File does not exist or is not a .py file.");
        return;
    }
    Command::new("python3")
        .arg(file_name)
        .status()
        .expect("Failed to run .py file.");
}

fn run_z(file_name: &str) {
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

fn run_n(file_name: &str) {
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

fn run_rust(file_name: &str) {
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

fn run_kotlin(file_name: &str) {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: run_file <filename>");
        std::process::exit(1);
    }

    let file_name = &args[1];

    match file_name.as_str() {
        f if f.ends_with(".c") => run_c(f),
        f if f.ends_with(".cpp") => run_cpp(f),
        f if f.ends_with(".cs") => run_cs(f),
        f if f.ends_with(".rb") => run_rb(f),
        f if f.ends_with(".hx") => run_hx(f),
        f if f.ends_with(".go") => run_golang(f),
        f if f.ends_with(".jl") => run_jul(f),
        f if f.ends_with(".java") => run_java(f),
        f if f.ends_with(".py") => run_py(f),
        f if f.ends_with(".zig") => run_z(f),
        f if f.ends_with(".nim") => run_n(f),
        f if f.ends_with(".rs") => run_rust(f),
        f if f.ends_with(".kt") => run_kotlin(f),
        _ => eprintln!("Error: Unsupported file type."),
    }
}
