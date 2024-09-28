use serde_json::Value;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::Command;

// Download and convert function with editable metadata
fn download_and_convert_to_m4a(youtube_link: &str, is_playlist: bool, audio_quality: u32) {
    let output_dir = "out";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Fetch and allow editing of metadata
    let metadata = fetch_metadata(youtube_link);
    let parsed_metadata = parse_metadata_to_editable_format(&metadata);
    let edited_metadata = edit_metadata_in_editor(&parsed_metadata);

    // Convert edited metadata back to JSON and save in a format `yt-dlp` can use
    let updated_metadata = convert_edited_metadata_to_json(&edited_metadata);

    // Prepare the `yt-dlp` command
    let mut command = Command::new("yt-dlp");
    command.args([
        "-x",
        "--audio-format",
        "m4a",
        "--audio-quality",
        &audio_quality.to_string(),
        "--add-metadata",
        "-o",
    ]);

    if is_playlist {
        print!("Enter the playlist name: ");
        io::stdout().flush().unwrap();
        let mut playlist_name = String::new();
        io::stdin()
            .read_line(&mut playlist_name)
            .expect("Failed to read input");

        let playlist_name = sanitize_directory_name(playlist_name.trim());
        let playlist_dir = format!("{}/{}", output_dir, playlist_name);
        fs::create_dir_all(&playlist_dir).expect("Failed to create playlist directory");
        command.args([
            &format!("{}/%(playlist_index)s - %(title)s.%(ext)s", playlist_dir),
            "--yes-playlist",
        ]);
    } else {
        command.arg(format!("{}/%(title)s.%(ext)s", output_dir));
    }

    command.arg(youtube_link);
    command.env("YT_DLP_METADATA", updated_metadata);

    let command_output = command.output().expect("Failed to execute yt-dlp command");

    if command_output.status.success() {
        println!("Download and conversion complete!");
    } else {
        eprintln!("Download and conversion failed!");
        if let Ok(stderr) = String::from_utf8(command_output.stderr) {
            eprintln!("yt-dlp error message:\n{}", stderr);
        }
    }
}

// Fetch metadata using yt-dlp
fn fetch_metadata(youtube_link: &str) -> String {
    let output = Command::new("yt-dlp")
        .args(["--dump-json", youtube_link])
        .output()
        .expect("Failed to fetch metadata");

    if output.status.success() {
        String::from_utf8(output.stdout).unwrap()
    } else {
        eprintln!("Failed to fetch metadata.");
        String::new()
    }
}

// Parse JSON metadata into a key-value format
fn parse_metadata_to_editable_format(metadata: &str) -> String {
    let parsed_json: Value = serde_json::from_str(metadata).expect("Invalid JSON format");
    let mut formatted_metadata = String::new();

    if let Some(title) = parsed_json["title"].as_str() {
        formatted_metadata.push_str(&format!("Title: {}\n", title));
    }
    if let Some(artist) = parsed_json["uploader"].as_str() {
        formatted_metadata.push_str(&format!("Artist: {}\n", artist));
    }
    if let Some(album) = parsed_json["album"].as_str() {
        formatted_metadata.push_str(&format!("Album: {}\n", album));
    }
    if let Some(genre) = parsed_json["tags"].as_array() {
        if !genre.is_empty() {
            formatted_metadata.push_str(&format!("Genre: {}\n", genre[0].as_str().unwrap_or("")));
        }
    }
    formatted_metadata
}

// Open metadata in `nano` for editing
fn edit_metadata_in_editor(metadata: &str) -> HashMap<String, String> {
    let temp_file_path = "/tmp/yt_metadata.txt";
    let mut file = File::create(temp_file_path).expect("Failed to create temp file");
    file.write_all(metadata.as_bytes())
        .expect("Failed to write metadata to file");

    // Open `nano` to edit metadata
    Command::new("nano")
        .arg(temp_file_path)
        .status()
        .expect("Failed to open editor");

    // Read edited metadata
    let edited_content =
        std::fs::read_to_string(temp_file_path).expect("Failed to read edited metadata");
    parse_edited_metadata(edited_content)
}

// Convert key-value formatted metadata back into JSON
fn convert_edited_metadata_to_json(edited_metadata: &HashMap<String, String>) -> String {
    let mut updated_metadata = HashMap::new();

    if let Some(title) = edited_metadata.get("Title") {
        updated_metadata.insert("title", title.clone());
    }
    if let Some(artist) = edited_metadata.get("Artist") {
        updated_metadata.insert("uploader", artist.clone());
    }
    if let Some(album) = edited_metadata.get("Album") {
        updated_metadata.insert("album", album.clone());
    }
    if let Some(genre) = edited_metadata.get("Genre") {
        updated_metadata.insert("tags", genre.clone());
    }

    serde_json::to_string(&updated_metadata).expect("Failed to convert edited metadata to JSON")
}

// Parse edited key-value metadata into a HashMap
fn parse_edited_metadata(edited_content: String) -> HashMap<String, String> {
    let mut metadata_map = HashMap::new();
    for line in edited_content.lines() {
        if let Some((key, value)) = line.split_once(": ") {
            metadata_map.insert(key.to_string(), value.to_string());
        }
    }
    metadata_map
}

// Sanitize directory names
fn sanitize_directory_name(name: &str) -> String {
    let invalid_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    name.chars()
        .filter(|&c| !invalid_chars.contains(&c))
        .collect::<String>()
        .trim()
        .to_owned()
}

// Main function
fn main() {
    let mut youtube_link = String::new();

    print!("Enter the YouTube link or playlist link: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut youtube_link)
        .expect("Failed to read input");

    let youtube_link = youtube_link.trim();
    let is_playlist = youtube_link.contains("playlist");

    let mut audio_quality_input = String::new();
    print!("Enter audio quality (0 - best, 9 - worst): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut audio_quality_input)
        .expect("Failed to read input");

    let audio_quality: u32 = audio_quality_input.trim().parse().unwrap_or(0);

    download_and_convert_to_m4a(youtube_link, is_playlist, audio_quality);
}
