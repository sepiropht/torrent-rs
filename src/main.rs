extern crate simple_bencode;
use url::{Url, Host};
use std::fs::File;
use std::io::prelude::*;
use std::net::{SocketAddr, UdpSocket};

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
            for (book, review) in &dico {
              //  println!("{:?}: \"{:?}\"", String::from_utf8_lossy(book), review);
            }
            announce = simple_bencode::decoding_helpers::pop_value_utf8_string(
                &mut dico,
                String::from("announce"),
            )
            .expect("String");
        }
        _ => println!("nothing"),
    }
    println!("traker {}", announce);
    let url= Url::parse(&announce).unwrap();
    println!("port {:?}, host {:?}", url.port(), url.host());
    let url = Url::parse("https://example.net").expect("url");

    let mut socket = UdpSocket::bind("127.0.0.1:8084").expect("port and adress");

    println!("Listening on {}", socket.local_addr()?);

    let msg = "hello world";
    println!("<- {}", msg);
    socket.connect("127.0.0.1:8084").expect("connect function failed");
    socket.send(msg.as_bytes()).expect("send hello !");

    let mut buf = vec![0u8; 1024];
    socket.recv_from(&mut buf)?;
    println!("-> {}\n", String::from_utf8_lossy(&mut buf));

    Ok(())
}
