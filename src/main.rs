// Pseudo-Roadmap

// ### Main Command

// - `musicman` (short for "music manager"): The main entry point for the CLI application.

// ### Commands and Subcommands

// 1. **Add**: Add new music files or directories to the library.
//    - `musicman add <path_to_file_or_directory>`: Adds the specified file or directory.
//    - Flags:
//      - `--recursive` or `-r`: Recursively add music files from directories.

// 2. **List**: List music files in the library.
//    - `musicman list`: Lists all files.
//    - Subcommands:
//      - `albums`: Lists all albums.
//      - `artists`: Lists all artists.
//      - `genres`: Lists music by genre.
//    - Flags:
//      - `--sort-by` (`name`, `date`, `artist`, etc.): Sorts the list according to the specified criterion.

// 3. **Play**: Play music.
//    - `musicman play <criteria>`: Plays music based on the specified criteria (could be a song name, album, artist, or genre).
//    - Subcommands:
//      - `shuffle`: Plays songs in shuffle mode.
//      - `album <album_name>`: Plays an entire album.
//      - `artist <artist_name>`: Plays all songs by a specific artist.
//      - `playlist <playlist_name>`: Plays a specific playlist.

// 4. **Search**: Search for music in the library.
//    - `musicman search <query>`: Searches the library for the query, which could be part of a song title, album, or artist name.

// 5. **Remove**: Remove music files or directories from the library.
//    - `musicman remove <path_to_file_or_directory>`: Removes the specified file or directory from the library.

// 6. **Create**: Create a new playlist or album.
//    - `musicman create playlist <playlist_name>`: Creates a new playlist.
//    - `musicman create album <album_name>`: Creates a new album grouping in the library.

// 7. **Edit**: Edit existing library items.
//    - `musicman edit metadata <path_to_file>`: Edits metadata for a specific file.
//    - Subcommands for editing:
//      - `playlist <playlist_name>`: Edit a playlist (add or remove songs).
//      - `album <album_name>`: Edit an album grouping (add or remove songs).

// 8. **Info**: Display information about

fn main() {
    println!("Hello, world!");
}
