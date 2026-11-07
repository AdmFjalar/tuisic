use std::{fs, path::Path, io};

fn main() -> io::Result<()>{


    let mut path = String::new(); // To be used for the playlist directory. This can be entered in the CLI.
    
    let stdin = io::stdin();
    stdin.read_line(&mut path)?;

    if Path::new(path.as_str().trim()).exists() {
        println!("Yippie! :D");
    } else {
        println!("Not found ;(");
    }
    Ok(())
}
