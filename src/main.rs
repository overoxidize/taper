// Pseudo-Roadmap

// ### Main Command

// - `taper` ("Terminal Audio PlayeER"): The main entry point for the CLI application.

// ### Commands and Subcommands

// 1. **Add**: Add new music files or directories to the library.
//    - `taper add <path_to_file_or_directory>`: Adds the specified file or directory.
//    - Flags:
//      - `--recursive` or `-r`: Recursively add music files from directories.

// 2. **List**: List music files in the library.
//    - `taper list`: Lists all files.
//    - Subcommands:
//      - `albums`: Lists all albums.
//      - `artists`: Lists all artists.
//      - `genres`: Lists music by genre.
//    - Flags:
//      - `--sort-by` (`name`, `date`, `artist`, etc.): Sorts the list according to the specified criterion.

// 3. **Play**: Play music.
//    - `taper play <criteria>`: Plays music based on the specified criteria (could be a song name, album, artist, or genre).
//    - Subcommands:
//      - `shuffle`: Plays songs in shuffle mode.
//      - `album <album_name>`: Plays an entire album.
//      - `artist <artist_name>`: Plays all songs by a specific artist.
//      - `playlist <playlist_name>`: Plays a specific playlist.

// 4. **Search**: Search for music in the library.
//    - `taper search <query>`: Searches the library for the query, which could be part of a song title, album, or artist name.

// 5. **Remove**: Remove music files or directories from the library.
//    - `taper remove <path_to_file_or_directory>`: Removes the specified file or directory from the library.

// 6. **Create**: Create a new playlist or album.
//    - `taper create playlist <playlist_name>`: Creates a new playlist.
//    - `taper create album <album_name>`: Creates a new album grouping in the library.

// 7. **Edit**: Edit existing library items.
//    - `taper edit metadata <path_to_file>`: Edits metadata for a specific file.
//    - Subcommands for editing:
//      - `playlist <playlist_name>`: Edit a playlist (add or remove songs).
//      - `album <album_name>`: Edit an album grouping (add or remove songs).

// 8. **Info**: Display information about
mod commands;
use clap::{Arg, Args, Command, Parser, Subcommand};
use commands::play;
use rodio::{
    source::{self, SineWave, Source},
    Decoder, OutputStream, Sink,
};
use std::fs::{DirEntry, File};
use std::io::BufReader;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Parser)]
#[clap(
    name = "Taper",
    about = "Taper is a terminal audio player.",
    version = "0.0.1"
)]
// #[command(multicall = true)]
pub struct Taper {
    #[clap(subcommand)]
    commands: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[clap(arg_required_else_help = true)]
    /// Plays a song or a playlist (expected to be a folder of songs)
    Play(PlayArgs),
    /// Pauses the playback of the currently playing song.
    Pause,
    /// Resumes the playback of the currently paused song.
    Resume,
    /// Ends the playback of the currently playing/paused song.
    Stop,
    /// Skips to the next song in the playlist.
    Next,
    /// Skips back to the previous song in the playlist.
    Previous,
    /// Exits the application.
    Exit,
}

#[derive(Debug, Args)]
struct PlayArgs {
    #[clap(short, long, value_name = "FILE(S)")]
    path: PathBuf,
    #[clap(short, long)]
    shuffle: bool,
    #[clap(short, long)]
    repeat: bool,
}
fn main() -> Result<(), String> {
    let taper = Taper::parse();
    dbg!(&taper);
    loop {
        match taper.commands {
            SubCommands::Play(play_args) => {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                // OutputStream is a wrapper over the `cpal` `Stream` struct, which represents
                // an open flow of audio data.
                let sink = Sink::try_new(&stream_handle).unwrap();
                let audio_source = play_args.path;
                let mut source_paths: Vec<DirEntry> = Vec::new();
                // The general idea here is that if we have a playlist,
                // we'll need to store a representation of its contents,
                // for shuffling or repeating.
                // Even if neither is selected, the default is to play
                // through songs sequentially, so they'll still need
                // to be represented and stored in some way.
                match audio_source.is_dir() {
                    true => match (play_args.repeat, play_args.shuffle) {
                        (true, true) => {}
                        (true, false) => {}
                        (false, true) => {}
                        (false, false) => {}
                    },
                    false => match play_args.repeat {
                        true => {}
                        _ => unimplemented!(),
                    },
                }


                sink.sleep_until_end();
            }
            SubCommands::Exit => {
                write!(stdout(), "Exiting...").map_err(|e| e.to_string())?;
                stdout().flush().map_err(|e| e.to_string())?;
            }
            _ => {
                println!("Not done yet.");
            }
        }
        return Ok(());
    }
}
