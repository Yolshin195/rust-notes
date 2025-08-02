use std::error::Error;

/// Plays an MP3 file from start to finish.
///
/// This function opens an audio file and plays it completely.
/// The program waits until the music ends before continuing.
///
/// # Arguments
///
/// * `file_name` - Path to the MP3 file (example: "music/song.mp3")
///
/// # Returns
///
/// * `Ok(())` - If the file plays successfully
/// * `Err` - If the file cannot be opened or played
///
/// # Example
///
/// ```rust
/// play_mp3_file("assets/finish.mp3".to_string())?;
/// ```
fn play_mp3_file(file_name: String) -> Result<(), Box<dyn Error>> {
    // Start audio system
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;

    // Create a new audio player
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    // Open the MP3 file
    let file = std::fs::File::open(file_name)?;

    // Add the music to the player
    sink.append(rodio::Decoder::try_from(file)?);

    // Wait until the music ends
    sink.sleep_until_end();

    // Everything is OK
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    play_mp3_file("assets/finish.mp3".to_string())
}
