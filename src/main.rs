use dialoguer::{Confirm, Input};
//use std::fs;
//use std::io::{self, Write};
use std::process::{Command, Stdio};

fn download_video(youtube_link: &str, output_file: &str) -> Result<(), String> {
    // Execute yt-dlp command for downloading the video/audio
    let mut command = Command::new("yt-dlp")
        .arg("-x") // Extract audio
        .arg("--audio-format")
        .arg("m4a") // Convert to m4a format
        .arg("-o")
        .arg(output_file) // Save file with this name
        .arg(youtube_link) // Pass the YouTube link
        .stdout(Stdio::null()) // Hide output
        .spawn()
        .map_err(|e| format!("Failed to execute yt-dlp: {}", e))?;

    command.wait().map_err(|e| format!("yt-dlp error: {}", e))?;

    Ok(())
}

fn edit_metadata(title: &mut String, artist: &mut String) {
    println!("\nExtracted Metadata:");
    println!("Title: {}", title);
    println!("Artist: {}", artist);

    // Ask the user if they want to edit the title
    *title = Input::new()
        .with_prompt("Enter new title")
        .with_initial_text(title.clone())
        .interact_text()
        .unwrap();

    // Ask the user if they want to edit the artist name
    if Confirm::new()
        .with_prompt("Edit Artist?")
        .interact()
        .unwrap()
    {
        *artist = Input::new()
            .with_prompt("Enter new artist")
            .with_initial_text(artist.clone())
            .interact_text()
            .unwrap();
    }

    println!("\nUpdated Metadata:");
    println!("Title: {}", title);
    println!("Artist: {}", artist);
}

fn sanitize_filename(input: &str) -> String {
    // Remove invalid characters for filenames
    let invalid_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    input
        .chars()
        .filter(|c| !invalid_chars.contains(c))
        .collect()
}

fn main() {
    // Prompt for the YouTube link
    let youtube_link: String = Input::new()
        .with_prompt("Enter the YouTube link")
        .interact_text()
        .unwrap();

    // Simulated metadata extraction (replace this with actual extraction if needed)
    let mut title: String = "Rick Astley - Never Gonna Give You Up".to_string();
    let mut artist = "Rick Astley".to_string();

    // Let the user edit metadata
    edit_metadata(&mut title, &mut artist);

    // Sanitize filename for saving
    let filename = sanitize_filename(&format!("{} - {}.m4a", artist, title));

    // Start downloading the video/audio
    println!("\nDownloading...");
    match download_video(&youtube_link, &filename) {
        Ok(_) => {
            println!("\nDownload complete! Saved as '{}'", filename);
        }
        Err(e) => {
            
            {
                eprintln!("Error during download: {}", e);
            }
        }
    }
}
