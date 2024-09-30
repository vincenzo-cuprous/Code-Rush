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
mod build;             // Include the new build module

// Optimized function to run Julia files
fn run_jul(file_name: &str) {
    // Check if the file has a .jl extension and exists
    match fs::metadata(file_name) {
        Ok(metadata) => {
            if file_name.ends_with(".jl") && metadata.is_file() {
                let status = Command::new("julia")
                    .arg(file_name)
                    .status()
                    .expect("Failed to run .jl file.");
                
                if !status.success() {
                    eprintln!("Error: Julia execution failed.");
                }
            } else {
                eprintln!("Error: File is not a .jl file.");
            }
        }
        Err(_) => eprintln!("Error: File does not exist."),
    }
}

// Function to run Zig files
fn run_zig(file_name: &str) {
    if !file_name.ends_with(".zig") || !fs::metadata(file_name).is_ok() {
        eprintln!("Error: File does not exist or is not a .zig file.");
        return;
    }
    zig_haxe_nim::run_zig(file_name);
}

// Function to run Haxe files
fn run_haxe(file_name: &str) {
    if !file_name.ends_with(".hx") || !fs::metadata(file_name).is_ok() {
        eprintln!("Error: File does not exist or is not a .hx file.");
        return;
    }
    zig_haxe_nim::run_haxe(file_name);
}

// Function to run Nim files
fn run_nim(file_name: &str) {
    if !file_name.ends_with(".nim") || !fs::metadata(file_name).is_ok() {
        eprintln!("Error: File does not exist or is not a .nim file.");
        return;
    }
    zig_haxe_nim::run_nim(file_name);
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
    if fs::metadata(&config_path).is_ok() {
        fs::read_to_string(config_path).unwrap_or_else(|_| "default".to_string())
    } else {
        "default".to_string()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the help flag `--h` is provided
    if args.len() == 2 && args[1] == "--h" {
        help::display_help();
        return;
    }

    // Check if the version flag `--version` is provided
    if args.len() == 2 && args[1] == "--version" {
        version::display_version();
        return;
    }

    // Check if the `-c` flag is used for setting the C# compiler
    if args.len() == 3 && args[1] == "-c" {
        let compiler = &args[2];
        set_compiler(compiler);
        println!("Compiler set to: {}", compiler);
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
        f if f.ends_with(".cs") => {
            if compiler == "dotnet" {
                c_cpp_cs::run_dot(f);
            } else if compiler == "mono" {
                c_cpp_cs::run_cs(f);
            } else {
                eprintln!("Error: Unsupported C# compiler configuration.");
            }
        },
        f if f.ends_with(".java") => java_kotlin_python::run_java(f),
        f if f.ends_with(".kt") => java_kotlin_python::run_kotlin(f),
        f if f.ends_with(".py") => java_kotlin_python::run_py(f),
        f if f.ends_with(".zig") => run_zig(f),
        f if f.ends_with(".hx") => run_haxe(f),
        f if f.ends_with(".nim") => run_nim(f),
        f if f.ends_with(".mojo") => mojo::run_mojo(f),
        f if f.ends_with(".rs") => rust_go_ruby::run_rust(f),
        f if f.ends_with(".go") => rust_go_ruby::run_golang(f),
        f if f.ends_with(".rb") => rust_go_ruby::run_rb(f),
        f if f.ends_with(".jl") => run_jul(f),  // Added Julia handling here
        _ => eprintln!("Unsupported file type."),
    }
}
