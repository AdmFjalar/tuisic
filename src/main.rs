use std::{fs::{self, File}, thread, sync::mpsc, io::{self, BufReader, Cursor}, path::Path, thread::sleep, time::Duration};
use rodio::{MixerDeviceSink, Player, Decoder};
use rodio::source::{SineWave, Source};

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

    // Instantiate channel for threads
    let (tx, rx) = mpsc::channel();

    // Instantiate rodio player
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("");
    let player = rodio::Player::connect_new(handle.mixer());

    // Get dir path
    let mut buffer = String::new();
    
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let path = &buffer.as_str().trim();

    // Spawn input thread to allow input during playback
    thread::spawn(move || {
        while(true) {
            let mut playback_buffer = String::new();
            stdin.read_line(&mut playback_buffer).expect("");
            let mut command = &playback_buffer.as_str().trim();

            if (command == &"pause") {
                tx.send("pause").unwrap();
            }
            else if (command == &"play") {
                tx.send("play").unwrap();
            }
        }
    });

    if Path::new(path).exists() {
        println!("Yippie! :D");

        // Loop through all files in directory and add append them to player
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file = File::open(entry_path)?;
            player.append(rodio::Decoder::try_from(file).expect(""));
        }

        player.set_speed(1.0);
        player.set_volume(1.0);
        

        // Infinite loop to continously receive playback commands
        while (true) {
            let received: &str = rx.recv().unwrap();
            if (received == "pause") {
            player.pause();
            }
            if (received == "play") {
                player.play();
            }
        }
    
        player.sleep_until_end();
    } else {
        println!("Not found ;(");
    }
    Ok(())
}
