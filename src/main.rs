extern crate simple_bencode;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("ubuntu.torrent")?;
    //    let mut Map = HashMap::new();
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    let res = simple_bencode::decode(&contents).expect("merde");
    let mut announce = String::from("");
    match res {
        simple_bencode::Value::Dictionary(dico) => {
            let mut dico = dico;
            announce = simple_bencode::decoding_helpers::pop_value_utf8_string(
                &mut dico,
                String::from("announce"),
            )
            .expect("String");
        }
        _ => println!("nothing"),
    }
    println!("traker {}", announce);
    Ok(())
}
