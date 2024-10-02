use std::io::{self, Write};
use tokio::process::Command as AsyncCommand;

#[tokio::main]
async fn main() {
    // Step 1: Prompt the user for the YouTube URL
    let mut video_url = String::new();
    println!("Enter the YouTube video or playlist URL:");
    io::stdin()
        .read_line(&mut video_url)
        .expect("Failed to read the video URL");
    let video_url = video_url.trim();

    // Step 2: Check if the URL is a playlist using yt-dlp
    let output_check = AsyncCommand::new("yt-dlp")
        .arg("--print")
        .arg("%(playlist_title)s")
        .arg(video_url)
        .output()
        .await
        .expect("Failed to check if URL is a playlist");

    let _is_playlist = !String::from_utf8_lossy(&output_check.stdout)
        .trim()
        .is_empty();

    // Step 3: Define the output template
    let output_template = "out/%(title)s.%(ext)s".to_string();

    // Step 4: Download the video or playlist using yt-dlp
    let download_status = AsyncCommand::new("yt-dlp")
        .arg("-x") // Extract audio
        .arg("--audio-format")
        .arg("m4a")
        .arg("-o")
        .arg(output_template)
        .arg(video_url)
        .status()
        .await
        .expect("Failed to execute yt-dlp");

    if !download_status.success() {
        eprintln!("Failed to download the audio");
        return;
    }

    // Step 5: Extract metadata for editing using yt-dlp
    let metadata_output = AsyncCommand::new("yt-dlp")
        .arg("--print")
        .arg("%(title)s,%(artist)s,%(album)s")
        .arg(video_url)
        .output()
        .await
        .expect("Failed to retrieve metadata");

    if !metadata_output.status.success() {
        eprintln!("Failed to extract metadata");
        return;
    }

    // Step 6: Parse metadata and prompt for editing
    let metadata_str = String::from_utf8_lossy(&metadata_output.stdout);
    let metadata_fields: Vec<&str> = metadata_str.split(',').collect();

    if metadata_fields.len() != 3 {
        eprintln!("Failed to parse metadata correctly");
        return;
    }

    let mut track_name = metadata_fields[0].trim().to_string();
    let mut artist_name = metadata_fields[1].trim().to_string();
    let mut album_name = metadata_fields[2].trim().to_string();

    // Display current metadata
    println!("\nCurrent Metadata:");
    println!("Track Name: {}", track_name);
    println!("Artist: {}", artist_name);
    println!("Album: {}", album_name);

    // Prompt for new metadata
    println!("\nEnter new metadata values (leave empty to keep current):");

    let mut new_track_name = String::new();
    print!("Track Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_track_name).unwrap();

    if !new_track_name.trim().is_empty() {
        track_name = new_track_name.trim().to_string();
    }

    let mut new_artist_name = String::new();
    print!("Artist: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_artist_name).unwrap();

    if !new_artist_name.trim().is_empty() {
        artist_name = new_artist_name.trim().to_string();
    }

    let mut new_album_name = String::new();
    print!("Album: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_album_name).unwrap();

    if !new_album_name.trim().is_empty() {
        album_name = new_album_name.trim().to_string();
    }

    // Step 7: Apply metadata using AtomicParsley
    let output_file = format!("out/{}.m4a", track_name);
    let metadata_command = AsyncCommand::new("atomicparsley")
        .arg(&output_file)
        .arg("--artist")
        .arg(&artist_name)
        .arg("--album")
        .arg(&album_name)
        .arg("--title")
        .arg(&track_name)
        .arg("--overWrite") // Overwrite the existing file
        .output()
        .await
        .expect("Failed to execute AtomicParsley");

    if !metadata_command.status.success() {
        eprintln!("Failed to apply metadata to the audio file.");
    } else {
        println!("\nMetadata successfully updated for {}", output_file);
    }
}
