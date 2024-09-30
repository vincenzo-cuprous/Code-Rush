use std::process::Command;
use std::path::Path;

// Make the build_mingw_cpp function public
pub fn build_mingw_cpp(file_name: &str) {
    // Check if the file is a .cpp file and if it exists
    if !file_name.ends_with(".cpp") || !Path::new(file_name).exists() {
        eprintln!("Error: File does not exist or is not a .cpp file.");
        return;
    }

    // Create the output filename by replacing the .cpp extension with .exe
    let output_file = file_name.replace(".cpp", ".exe");

    // Use the MinGW compiler (x86_64-w64-mingw32-g++) to compile the C++ file
    let status = Command::new("x86_64-w64-mingw32-g++")
        .arg(file_name)
        .arg("-o")
        .arg(&output_file)
        .status()
        .expect("Failed to execute MinGW C++ compiler.");

    // Check if the compilation was successful and provide appropriate feedback
    if status.success() {
        println!("Successfully compiled {} to {}", file_name, output_file);
    } else {
        eprintln!("Error: MinGW C++ compilation failed.");
    }
}

// Update the publish_dotnet function to accept a file name
pub fn publish_dotnet(file_name: &str) {
    // Execute the dotnet publish command, using the provided file name if necessary
    let output = Command::new("dotnet")
        .arg("publish")
        .arg("-c")
        .arg("Release")
        .arg("-r")
        .arg("win-x64")
        .arg("--self-contained")
        .arg(file_name) // If needed, modify the command to include the file name
        .output()
        .expect("Failed to execute dotnet publish command.");

    // Check if the command was successful and handle the output
    if output.status.success() {
        println!("C# project published successfully.");
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: Failed to publish C# project. {}", error_message);
    }
}

// ... (other existing functions in build_1.rs)
