use std::{fs::{self, File}, io::{self, BufReader, Cursor}, path::Path, time::Duration};
use rodio::{MixerDeviceSink, Player, Decoder};
use rodio::source::{SineWave, Source};

fn main() -> io::Result<()>{

    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("");
    let player = rodio::Player::connect_new(handle.mixer());

    let mut buffer = String::new();
    
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let path = buffer.as_str().trim();

    if Path::new(path).exists() {
        println!("Yippie! :D");
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file = File::open(entry_path)?;
            player.append(rodio::Decoder::try_from(file).expect(""));
        }
        player.sleep_until_end();
    } else {
        println!("Not found ;(");
    }
    Ok(())
}
