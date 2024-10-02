use std::collections::HashMap;
use std::io::{self};
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let video_url = "https://www.youtube.com/watch?v=YOUR_VIDEO_ID"; // Replace with your video URL
    let output = "downloaded_audio.m4a";

    // Step 1: Download the audio using yt-dlp
    let status = Command::new("yt-dlp")
        .arg("-x") // Extract audio
        .arg("--audio-format")
        .arg("m4a")
        .arg("-o")
        .arg(output)
        .arg(video_url)
        .status()
        .await
        .expect("Failed to execute yt-dlp");

    if !status.success() {
        eprintln!("Failed to download the audio");
        return;
    }

    // Step 2: Extract metadata using yt-dlp
    let output_metadata = Command::new("yt-dlp")
        .arg("--print")
        .arg("%(title)s,%(uploader)s,%(release_date)s,%(track)s,%(track_number)s,%(album)s,%(artist)s")
        .arg(video_url)
        .output()
        .await
        .expect("Failed to retrieve metadata");

    if !output_metadata.status.success() {
        eprintln!("Failed to extract metadata");
        return;
    }

    // Convert the metadata output to a string
    let metadata_str = String::from_utf8_lossy(&output_metadata.stdout);
    let metadata_keys = vec![
        "title", "uploader", "release_date", "track", "track_number", "album", "artist",
    ];
    
    let mut metadata: HashMap<&str, String> = HashMap::new();
    for (key, value) in metadata_keys.iter().zip(metadata_str.split(',')) {
        metadata.insert(*key, value.trim().to_string());
    }

    // Step 3: Display metadata for editing
    println!("Current Metadata:");
    for (key, value) in &metadata {
        println!("{}: {}", key, value);
    }

    // Step 4: Prompt user for metadata updates
    for key in metadata_keys {
        let mut new_value = String::new();
        println!("Enter new {} (leave empty to keep current):", key);
        io::stdin().read_line(&mut new_value).unwrap();
        if !new_value.trim().is_empty() {
            metadata.insert(key, new_value.trim().to_string());
        }
    }

    // Step 5: Print updated metadata
    println!("\nUpdated Metadata:");
    for (key, value) in &metadata {
        println!("{}: {}", key, value);
    }

    // Step 6: Apply the metadata using yt-dlp (or other tool)
    // For simplicity, we'll just print the metadata here, but you can use a tool like `ffmpeg`
    // to apply metadata to the downloaded file if needed.
    println!("\nUse a tool like `ffmpeg` or `mutagen` to apply the above metadata to your audio file.");
}
