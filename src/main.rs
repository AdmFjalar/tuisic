use std::{fs::{self, File}, thread, sync::mpsc, io::{self, BufReader, Cursor}, path::Path, thread::sleep, time::Duration};
use rodio::{MixerDeviceSink, Player, Decoder};
use rodio::source::{SineWave, Source};

fn main() -> io::Result<()>{

    let (tx, rx) = mpsc::channel();

    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("");
    let player = rodio::Player::connect_new(handle.mixer());

    // Get dir path
    let mut buffer = String::new();
    
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let path = &buffer.as_str().trim();

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
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file = File::open(entry_path)?;
            player.append(rodio::Decoder::try_from(file).expect(""));
        }
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
