use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use zip::ZipArchive;
use std::fs::File;
use std::io::{self, Read, Write};

fn unzip_file(zip_path: &str, extract_to: &str) -> io::Result<()> {
    let zip_path = Path::new(zip_path);
    let extract_to = Path::new(extract_to);

    if !zip_path.exists() {
        println!("ZIP file '{}' does not exist.", zip_path.display());
        return Ok(());
    }

    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(extract_to)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = extract_to.join(file.mangled_name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(p)?;
            }

            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    println!("Unzipped '{}' to '{}'.", zip_path.display(), extract_to.display());
    Ok(())
}

fn copy_directory(source_dir: &str, destination_dir: &str) -> io::Result<()> {
    let source_dir = Path::new(source_dir);
    let destination_dir = Path::new(destination_dir);

    if !source_dir.exists() {
        println!("Source directory '{}' does not exist.", source_dir.display());
        return Ok(());
    }

    if destination_dir.exists() {
        println!("Destination directory '{}' already exists. Overwriting...", destination_dir.display());
        fs::remove_dir_all(destination_dir)?;
    }

    fs::create_dir_all(destination_dir)?;

    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = destination_dir.join(entry.file_name());

        if path.is_dir() {
            copy_directory(path.to_str().unwrap(), dest_path.to_str().unwrap())?;
        } else {
            fs::copy(&path, dest_path)?;
        }
    }

    println!("Directory successfully copied from '{}' to '{}'.", source_dir.display(), destination_dir.display());
    Ok(())
}

fn create_shortcut(target_path: &str, shortcut_path: &str) -> io::Result<()> {
    let target_path = Path::new(target_path);
    let shortcut_path = Path::new(shortcut_path);

    let mut cmd = Command::new("powershell");
    cmd.args(&[
        "-Command",
        &format!("$WshShell = New-Object -ComObject WScript.Shell; $shortcut = $WshShell.CreateShortcut('{}'); $shortcut.TargetPath = '{}'; $shortcut.Save()", shortcut_path.display(), target_path.display())
    ]);

    cmd.status()?;
    
    println!("Shortcut created at '{}'.", shortcut_path.display());
    Ok(())
}

fn cleanup_directory(directory: &str) -> io::Result<()> {
    let directory = Path::new(directory);

    if directory.exists() {
        fs::remove_dir_all(directory)?;
        println!("Cleaned up temporary directory '{}'.", directory.display());
    } else {
        println!("Directory '{}' does not exist. Nothing to clean up.", directory.display());
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Paths for the zip file, extraction, destination, and shortcut
    let zip_path = "coderush_x86_64_windows.zip"; // Path to the ZIP file
    let extract_to = "extracted_folder"; // Temporary folder to unzip contents
    let destination_dir = r"C:\Program Files\Coderush"; // Destination path on Windows
    let shortcut_path = format!(r"C:\Users\{}\Desktop\Coderush.lnk", env::var("USERNAME").unwrap()); // Shortcut on Desktop
    let target_path = r"C:\Program Files\Coderush\coderush.exe"; // Path to the executable

    // Unzip the file
    unzip_file(zip_path, extract_to)?;

    // Copy the extracted folder to the destination
    copy_directory(extract_to, destination_dir)?;

    // Create a shortcut to the executable
    create_shortcut(target_path, &shortcut_path)?;

    // Cleanup: Remove the extracted folder
    cleanup_directory(extract_to)?;

    Ok(())
}
