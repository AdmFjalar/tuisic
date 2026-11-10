use std::{fs::{self, File}, thread, sync::mpsc, io::{self, BufReader, Cursor, stdout}, path::Path, thread::sleep, time::Duration};
use rodio::{MixerDeviceSink, Player, Decoder};
use rodio::source::{SineWave, Source};
use crossterm::{cursor, execute, terminal::{self, Clear, ClearType::{All, Purge}, disable_raw_mode, enable_raw_mode}};
use crossterm::event::{self, Event, KeyEvent, KeyCode, KeyModifiers};
use crossterm::style::Stylize;

struct Entry {
    path: String,
    // name: String,
    // timestamp: Duration,
    // length: Duration,
}

struct Playlist {
    path: String,
    songs: Vec<Entry>,
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
    let mut current_song: Entry;
    let mut queued_songs: Vec<Entry>;
    let mut cur_path = String::from("/home/oliver/");

    enable_raw_mode()?;

    
    // Instantiate rodio player
    let mut handle = rodio::DeviceSinkBuilder::open_default_sink().expect("");
    handle.log_on_drop(false);
    let player = rodio::Player::connect_new(handle.mixer());


    let mut path = String::new();


    let mut sel_index = 0;
    let mut last_index = 999;
    loop {
        path = cur_path.to_string();
        if let Ok(files) = get_directory_contents(&path.trim()) {
            if (last_index != sel_index) {
                print_menu(sel_index, last_index, &files);
            }

            last_index = sel_index;
            if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent{ code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Up => {if (sel_index == 0) {
                        sel_index = files.len() - 1;
                    } else {
                        sel_index -= 1
                    }},
                    KeyCode::Down => {sel_index += 1},
                    KeyCode::Right => {
                        if Path::new(&files[sel_index].path).is_dir() {
                        cur_path = files[sel_index].path.clone();
                        sel_index = 0;
                        last_index = 999;
                        } else {
                            let file = File::open(&files[sel_index].path)?;
                            player.stop();
                            player.append(rodio::Decoder::try_from(file).expect("Unsupported audio format"));
                        }
                    },
                    KeyCode::Left => {
                        
                        if (Path::new(&cur_path).parent().is_some()) {
                            cur_path = Path::new(&cur_path).parent().unwrap().display().to_string();
                        }
                        else {
                            break;
                        }
                        sel_index = 0;
                        last_index = 999;
                    }
                    _ => (),
                }
            }
            if (sel_index < 0) {
                sel_index = files.len() - 1;
            } else if (sel_index > files.len() - 1) {
                sel_index = 0;
            }
            }
        };
    }
   
    disable_raw_mode();
    Ok(())
}

fn get_directory_contents(dir_path: &str) -> std::io::Result<Vec<Entry>> {
    let mut files: Vec<Entry> = vec![];

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let file = Entry {
            path: entry.path().display().to_string(),
        };
        files.push(file);
    }

    Ok(files)
}

fn print_menu(sel_index: usize, last_index: usize, files: &Vec<Entry>) {
    execute!(stdout(), Clear(All)).unwrap();
    for i in 0..files.len() {
        if i == sel_index.try_into().unwrap() {
            print!("> {}\r\n", files[i].path.clone().bold().italic());
        } else {
            print!("  {}\r\n", files[i].path.clone());
        }
    }
}