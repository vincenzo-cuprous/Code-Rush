use std::env;
use std::fs;
use std::process::Command;

mod c_cpp_cs;          // Include the c_cpp_cs module
mod zig_haxe_nim;      // Include the zig_haxe_nim module
mod java_kotlin_python; // Include the java_kotlin_python module
mod rust_go_ruby;      // Include the rust_go_ruby module
mod mojo;              // Include the mojo module
mod help;              // Include the help module
mod version;           // Include the version module

// Function to check file validity based on extension
fn is_valid_file(file_name: &str, ext: &str) -> bool {
    file_name.ends_with(ext) && fs::metadata(file_name).is_ok()
}

// Optimized function to run Julia files
fn run_jul(file_name: &str) {
    if is_valid_file(file_name, ".jl") {
        let status = Command::new("julia")
            .arg(file_name)
            .status()
            .expect("Failed to run .jl file.");
        if !status.success() {
            eprintln!("Error: Julia execution failed.");
        }
    } else {
        eprintln!("Error: File is not a .jl file or does not exist.");
    }
}

// Functions to run Zig, Haxe, and Nim files
fn run_zhn(file_name: &str, lang: &str) {
    if !is_valid_file(file_name, lang) {
        eprintln!("Error: File does not exist or is not a {} file.", lang);
        return;
    }

    match lang {
        ".zig" => zig_haxe_nim::run_zig(file_name),
        ".hx" => zig_haxe_nim::run_haxe(file_name),
        ".nim" => zig_haxe_nim::run_nim(file_name),
        _ => unreachable!(),
    }
}

// Function to set the compiler configuration for C#
fn set_compiler(compiler: &str) {
    let config_path = dirs::config_dir().unwrap().join("coderush/compiler_config.txt");
    fs::create_dir_all(config_path.parent().unwrap()).expect("Failed to create config directory.");
    fs::write(config_path, compiler).expect("Failed to write compiler configuration.");
}

// Function to get the current compiler configuration for C#
fn get_compiler() -> String {
    let config_path = dirs::config_dir().unwrap().join("coderush/compiler_config.txt");
    fs::read_to_string(&config_path).unwrap_or_else(|_| "default".to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for help or version flags
    if args.len() == 2 {
        match args[1].as_str() {
            "--h" => {
                help::display_help();
                return;
            }
            "--version" => {
                version::display_version();
                return;
            }
            _ => { // Wildcard case to handle invalid arguments
                eprintln!("Invalid argument: {}", args[1]);
                std::process::exit(1);
            }
        }
    }

    // Check if the `-c` flag is used for setting the C# compiler
    if args.len() == 3 && args[1] == "-c" {
        let compiler = &args[2];
        set_compiler(compiler);
        println!("Compiler set to: {}", compiler);
        return;
    }

    // Check if the `-w` flag is used to build a Windows executable using MinGW
    if args.len() >= 3 && args[1] == "-w" {
        let file_name = &args[2];
        if fs::metadata(file_name).is_err() {
            eprintln!("Error: File '{}' does not exist.", file_name);
            return;
        }

        if file_name.ends_with(".c") {
            build::build_mingw(file_name);  // Compile C files using MinGW GCC
        } else if file_name.ends_with(".cpp") {
            build_1::build_mingw_cpp(file_name);  // Compile C++ files using MinGW G++
        } else {
            eprintln!("Error: Unsupported file type with -w flag. Only .c and .cpp files are allowed.");
        }
        return;
    }

    // Handle the build command
    if args.len() == 3 && args[1] == "build" {
        let file_name = &args[2];
        build::build_file(file_name);
        return;
    }

    // Ensure a valid filename is provided
    if args.len() != 2 {
        eprintln!("Usage: coderush <filename> or coderush build <filename> or coderush -c <compiler>");
        std::process::exit(1);
    }

    let file_name = &args[1];
    let compiler = get_compiler();

    // File execution logic based on the file extension
    match file_name.as_str() {
        f if f.ends_with(".c") => c_cpp_cs::run_c(f),
        f if f.ends_with(".cpp") => c_cpp_cs::run_cpp(f),
        f if f.ends_with(".java") => java_kotlin_python::run_java(f),
        f if f.ends_with(".kt") => java_kotlin_python::run_kotlin(f),
        f if f.ends_with(".py") => java_kotlin_python::run_py(f),
        f if f.ends_with(".zig") => run_zhn(f, ".zig"),
        f if f.ends_with(".hx") => run_zhn(f, ".hx"),
        f if f.ends_with(".nim") => run_zhn(f, ".nim"),
        f if f.ends_with(".mojo") => mojo::run_mojo(f),
        f if f.ends_with(".rs") => rust_go_ruby::run_rust(f),
        f if f.ends_with(".go") => rust_go_ruby::run_golang(f),
        f if f.ends_with(".rb") => rust_go_ruby::run_rb(f),
        f if f.ends_with(".jl") => run_jul(f),  // Added Julia handling here
        f if f.ends_with(".cs") => c_cpp_cs::run_cs(f), // Added C# handling here
        _ => eprintln!("Unsupported file type."), // Wildcard case to handle unsupported types
    }
}
