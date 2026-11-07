use std::{fs::{self, File}, thread, sync::mpsc, io::{self, BufReader, Cursor}, path::Path, thread::sleep, time::Duration};
use rodio::{MixerDeviceSink, Player, Decoder};
use rodio::source::{SineWave, Source};

struct Song {
    path: String,
    name: String,
    timestamp: Duration,
    length: Duration,
}

struct Playlist {
    path: String,
    songs: Vec<Song>,
}

fn main() -> io::Result<()>{

    // IDEAS
    // 1. Queue music (add songs by file path). Could add easier navigation with arrow keys + enter key down the road.
    // 2. Shuffle order of songs. 
    // 3. Add repeat function (song or playlist).
    // 4. Playlists.
    // 5. MPRIS integration.
    // 6. Move queue from player to separate queue. This would allow for other ideas to more easily be added.
    // 7. Next / Previous songs.
    // 8. Jump around in song (with arrow keys?).

    let mut current_playlist: Playlist;
    let mut current_song: Song;

    // Instantiate channel for threads
    let (tx, rx) = mpsc::channel();

    // Instantiate rodio player
    let mut handle = rodio::DeviceSinkBuilder::open_default_sink().expect("");
    handle.log_on_drop(false);
    let player = rodio::Player::connect_new(handle.mixer());

    let stdin = io::stdin();

    // Spawn input thread to allow input during playback
    thread::spawn(move || {
        let mut buffer = String::new();
        loop {
            buffer.clear();
            stdin.read_line(&mut buffer).expect("");
            let command = buffer.trim().to_string();

            if tx.send(command).is_err() {
                break;
            }
        }
    });
    
    let mut directory_exists: bool = false;

    let mut path = String::new();
    while !Path::new(&path).exists() {
        path = rx.recv().unwrap().trim().to_string();
    }

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let file = File::open(entry.path())?;
        player.append(rodio::Decoder::try_from(file).expect("Unsupported audio format"));
    }

        player.set_speed(1.0);
        player.set_volume(1.0);
        
        // Infinite loop to continously receive playback commands
        loop {
            let received: String = rx.recv().unwrap();
            if (received == "pause") {
            player.pause();
            }
            if (received == "play") {
                player.play();
            }
            if (received == "next") {
                player.skip_one();
            }
            if (received == "close") {
                break;
            }
        }
    
        // player.sleep_until_end();
    
    Ok(())
}
