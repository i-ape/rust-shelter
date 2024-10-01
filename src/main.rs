// Use the necessary crates for audio processing and async command execution
use lofty::{file::TaggedFileExt, read_from_path};
use tokio::process::Command as AsyncCommand;

#[tokio::main]
async fn main() {
    let video_url = "https://www.youtube.com/watch?v=YOUR_VIDEO_ID"; // Replace with your video URL
    let output = "downloaded_audio.m4a";

    // Download the audio using yt-dlp
    let status = AsyncCommand::new("yt-dlp")
        .arg("-x") // Extract audio
        .arg("--audio-format")
        .arg("m4a") // Set audio format to m4a
        .arg("-o")
        .arg(output) // Specify output file name
        .arg(video_url)
        .status()
        .await
        .expect("Failed to execute yt-dlp");

    // Check if the download was successful
    if !status.success() {
        eprintln!("Failed to download the audio");
        return;
    }

    // Read the metadata from the downloaded audio file
    let mut audio_file = read_from_path(output).expect("Failed to open the audio file");
    let mut tags = audio_file.tags().expect("Failed to read tags");

    // Display existing tags for user reference
    println!("Current tags:");
    for (key, value) in tags.iter() {
        println!("{}: {}", key, value);
    }

    // Prompt for metadata editing
    let mut new_title = String::new();
    println!("Enter new title (leave empty to keep current):");
    std::io::stdin().read_line(&mut new_title).unwrap();

    // Update the title tag if a new title is provided
    if !new_title.trim().is_empty() {
        tags.insert("title".to_string(), new_title.trim().to_string());
    }

    // Write updated tags back to the audio file
    write_to_path(&audio_file, output).expect("Failed to write tags");

    println!("Updated tags successfully!");
}
