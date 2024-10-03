use regex::Regex;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

fn download_and_convert_to_m4a(youtube_link: &str, is_playlist: bool, audio_quality: u32) {
    let output_dir = "out";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let mut command = Command::new("yt-dlp");
    command.args([
        "-x",
        "--audio-format",
        "m4a",
        "--audio-quality",
        &audio_quality.to_string(),
        "--add-metadata",
        "--metadata-from-title",
        "%(artist)s - %(title)s",
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

    let mut child = command
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start yt-dlp command");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = io::BufReader::new(stdout);

    let _re_progress = Regex::new(r"(\d+\.\d+)%").unwrap(); // Capture percentage if needed
    let re_completed_files = Regex::new(r"\[\d+/\d+\]").unwrap(); // Capture completed files count

    println!("Downloading...");

    let mut completed_files = 0; // Counter for completed files

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                if let Some(captures) = re_completed_files.captures(&line_content) {
                    if let Some(_file_count) = captures.get(0) {
                        completed_files += 1;
                        print!("\rFiles Completed: {}{}", completed_files, " ".repeat(10)); // Clear line for neatness
                        io::stdout().flush().unwrap();
                    }
                }
            }
            Err(e) => eprintln!("Failed to read line: {}", e),
        }
    }

    let status = child.wait().expect("Failed to wait for yt-dlp process");
    if status.success() {
        println!("\nDownload and conversion complete!");
    } else {
        eprintln!("\nDownload and conversion failed!");
    }
}

fn sanitize_directory_name(name: &str) -> String {
    let invalid_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    let sanitized_name = name
        .chars()
        .filter(|&c| !invalid_chars.contains(&c))
        .collect::<String>();
    sanitized_name.trim().to_owned()
}

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
