use bendy::decoding::{Error, FromBencode, Object, ResultExt};
use dns_lookup::lookup_host;
use std::fs::File;
use std::io::Read;
use url::Url;

#[derive(Debug, PartialEq)]
struct Torrent {
    announce: String,
    //info: Info,
}

#[derive(Debug, PartialEq)]
struct Info {
    name: String,
    piece_length: i64,
    //pieces: Vec<u8>,
}
impl FromBencode for Info {
    const EXPECTED_RECURSION_DEPTH: usize = 1;

    fn decode_bencode_object(object: Object) -> Result<Self, Error> {
        let mut name = None;
        let mut piece_length: Option<i64> = None;
        //let mut pieces = Vec::new();

        let mut dict = object.try_into_dictionary()?;
        while let Some(pair) = dict.next_pair()? {
            match pair {
                (b"name", value) => {
                    name = String::decode_bencode_object(value)
                        .context("name")
                        .map(Some)?;
                }
                (b"piece_length", value) => {
                    piece_length = i64::decode_bencode_object(value)
                        .context("piece_length")
                        .map(Some)?;
                }
                // (b"pieces", value) => {
                //     pieces = vec::decode_bencode_object(value)
                //         .context("pieces")
                //         .map(Some)?;
                // }
                (unknown_field, _) => {
                    return Err(Error::unexpected_field(String::from_utf8_lossy(
                        unknown_field,
                    )));
                }
            }
        }

        let name = name.ok_or_else(|| Error::missing_field("name"))?;
        let piece_length = piece_length.ok_or_else(|| Error::missing_field("piece_length"))?;

        Ok(Info { name, piece_length })
    }
}

impl FromBencode for Torrent {
    const EXPECTED_RECURSION_DEPTH: usize = 3;

    fn decode_bencode_object(object: Object) -> Result<Self, Error> {
        let mut announce = None;
        //let mut info = None;

        let mut dict = object.try_into_dictionary()?;
        while let Some(pair) = dict.next_pair()? {
            match pair {
                (b"announce", value) => {
                    announce = String::decode_bencode_object(value)
                        .context("announce")
                        .map(Some)?;
                }
                // (b"info", value) => info = Some(Info::from_bencode(value)),
                (unknown_field, _) => {
                    // return Err(Error::unexpected_field(String::from_utf8_lossy(
                    //     unknown_field,
                    //)));
                    dbg!("Error field not expected", &unknown_field);
                }
            }
        }

        let announce = announce.ok_or_else(|| Error::missing_field("announce"))?;
        //let info = info.ok_or_else(|| Error::missing_field("label"))?;

        Ok(Torrent { announce })
    }
}

fn main() -> Result<(), Error> {
    // 2.
    let mut file = File::open("ubuntu.torrent").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let torrent = Torrent::from_bencode(&contents)?;
    println!("{:?}", torrent);

    let announce = &torrent.announce;
    print!("parsing success!!!!!!!!!!!!!!!!!!!!!, result :{}", announce);
    let url = Url::parse(announce).unwrap();

    // 3.
    let host = url.host_str().unwrap();
    let port = url.port().unwrap();

    use std::net::UdpSocket;

    // 1. Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    // 3. Send a message
    let message = b"hello world";
    dbg!("Montre moi ce qu'il y a ici", &host, &port);
    let ips: Vec<std::net::IpAddr> = lookup_host(host).unwrap();
    assert!(ips.contains(&"185.125.190.59".parse().unwrap()));
    let addr = format!("{}:{}", ips[1].to_string(), port);
    dbg!("IIIIIIIIIIIPPPP", &addr);
    socket
        .send_to(message, addr)
        .expect("Failed to send message");

    // 4. Receive a response
    let mut buf = [0; 1024];
    let (number_of_bytes, src_addr) = socket
        .recv_from(&mut buf)
        .expect("Failed to receive message");

    // 5. Print the response
    println!("Received {} bytes from {}", number_of_bytes, src_addr);
    println!("Message: {:?}", &buf[..number_of_bytes]);

    Ok(())
}
