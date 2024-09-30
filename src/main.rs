use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

// Function to prompt the user for metadata input
fn prompt_metadata() -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    println!("Please enter metadata for the files (leave blank to skip):");

    metadata.insert("Title".to_string(), prompt("Enter Title: "));
    metadata.insert("Artist".to_string(), prompt("Enter Artist: "));
    metadata.insert("Album".to_string(), prompt("Enter Album: "));
    metadata.insert("Genre".to_string(), prompt("Enter Genre: "));

    metadata
}

// Helper function for user input
fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

// Function to apply metadata to all m4a files in a specified directory
fn apply_metadata(directory: &Path, metadata: &HashMap<String, String>) {
    for entry in fs::read_dir(directory)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "m4a") {
            let mut args = vec!["-i", path.to_str().unwrap(), "-c", "copy"];

            for (_key, value) in metadata {
                if !value.is_empty() {
                    args.push("-metadata");
                    //args.push(&format!("{}={}", key.to_lowercase(), value));
                }
            }

            // Create output filename with "_updated" suffix
            let output_path = path.with_file_name(format!(
                "{}_updated.m4a",
                path.file_stem().unwrap().to_string_lossy()
            ));

            args.push(output_path.to_str().unwrap());

            let status = Command::new("ffmpeg")
                .args(&args)
                .status()
                .expect("Failed to run ffmpeg");

            if status.success() {
                println!("Successfully updated: {}", path.display());
            } else {
                eprintln!("Failed to update: {}", path.display());
            }
        }
    }
}

fn main() {
    // Prompt the user for directory path
    let dir_path = prompt("Enter the path to the folder containing the tracks: ");
    let directory = Path::new(&dir_path);

    if !directory.exists() || !directory.is_dir() {
        eprintln!("Invalid directory path.");
        return;
    }

    // Collect metadata from user
    let metadata = prompt_metadata();

    // Apply metadata to the files in the directory
    apply_metadata(directory, &metadata);
}
