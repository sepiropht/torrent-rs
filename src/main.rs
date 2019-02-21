extern crate bencode;
extern crate hyper;
extern crate url;

use bencode::util::ByteString;
use bencode::{Bencode, FromBencode};
use std::fs::File;
use std::io::prelude::*;
use std::net::UdpSocket;
mod decoder;
mod hash;

use self::hyper::header::Connection;
use self::hyper::Client;
use self::url::percent_encoding::{percent_encode };

fn main() -> std::io::Result<()> {
    let mut file = File::open("ubuntu.torrent")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let bencode: bencode::Bencode = bencode::from_vec(contents).unwrap();
    let result: Metainfo = FromBencode::from_bencode(&bencode).unwrap();
    dbg!(&result.announce);
    let url = format!("{}", result.announce);

    let mut client = Client::new();
    let mut http_res = client.get(&url).header(Connection::close()).send().unwrap();
    let mut body = Vec::new();
    http_res.read_to_end(&mut body).unwrap();
    dbg!(&body);

    Ok(())
}

use hash::{calculate_sha1, Sha1};

#[derive(PartialEq, Debug)]
pub struct Metainfo {
    pub announce: String,
    pub info: Info,
    pub info_hash: Vec<u8>,
    pub created_by: String,
}

impl FromBencode for Metainfo {
    type Err = decoder::Error;

    fn from_bencode(bencode: &bencode::Bencode) -> Result<Metainfo, decoder::Error> {
        match bencode {
            &Bencode::Dict(ref m) => {
                let info_bytes = get_field_as_bencoded_bytes!(m, "info");
                let info_hash = calculate_sha1(&info_bytes);

                let metainfo = Metainfo {
                    announce: get_field!(m, "announce"),
                    info: get_field!(m, "info"),
                    info_hash,
                    created_by: get_field_with_default!(m, "created by", "".to_string()),
                };
                Ok(metainfo)
            }
            _ => Err(decoder::Error::NotADict),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Info {
    pub piece_length: u32,
    pub pieces: Vec<Sha1>,
    pub num_pieces: u32,
    pub name: String,
    pub length: u64,
}

impl FromBencode for Info {
    type Err = decoder::Error;

    fn from_bencode(bencode: &bencode::Bencode) -> Result<Info, decoder::Error> {
        match bencode {
            &Bencode::Dict(ref m) => {
                let pieces_bytes = get_field_as_bytes!(m, "pieces");
                let pieces: Vec<Sha1> = pieces_bytes.chunks(20).map(|v| v.to_owned()).collect();
                let num_pieces = pieces.len() as u32;

                let info = Info {
                    piece_length: get_field!(m, "piece length"),
                    pieces: pieces,
                    num_pieces: num_pieces,
                    name: get_field!(m, "name"),
                    length: get_field!(m, "length"),
                };

                Ok(info)
            }
            _ => Err(decoder::Error::NotADict),
        }
    }
}
