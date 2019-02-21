use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs;
use bencode::util::ByteString;
extern crate bencode;
use bencode::{Bencode, FromBencode};
mod decoder;
mod hash;
/*
#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MyStruct {
        a: i32,
        b: String,
        c: Vec<u8>,
}
*/

fn main() -> std::io::Result<()> {
    let mut file = File::open("ubuntu.torrent")?;
    //println!("{:?}", buf_reader);
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    println!("{:?}", contents);
    let bencode: bencode::Bencode = bencode::from_vec(contents).unwrap();
    //let mut decoder = Decoder::new(&bencode);
    //let result = Decodable::decode(&mut decoder).unwrap();
    let result : Metainfo = FromBencode::from_bencode(&bencode).unwrap();
    dbg!(result);
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
