use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut file = File::open("ubuntu.torrent")?;
    //println!("{:?}", buf_reader);
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    println!("{:?}", contents);
    Ok(())
}
