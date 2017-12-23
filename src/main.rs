#[macro_use]
extern crate nom;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use nom::le_u32;

#[macro_use]
mod header;
mod crc32_rl;

#[macro_use]
use header::primitives::*;

//static REPLAY_FILE_STR: &'static str = "replays/7560D3FE446244A56C0EB198007F2B92.replay";
static REPLAY_FILE_STR: &'static str = "replays/1EBA9EA845DB4BD7809E78A7F4A7F1EC.replay";
//static REPLAY_FILE_STR: &'static str = "replays/F32599A54B1831A58C6C55A5334890AF.replay";
const DEFAULT_REPLAY_BUFFER: usize = 2 * 1024 * 1024;


fn parse(file: &str) -> io::Result<()> {
    let mut file = BufReader::new(File::open(file)?);

    let mut header_length_buf = [0u8; 4];
    let (_, header_length) = match file.read_exact(&mut header_length_buf) {
        Ok(_) => le_u32(&mut header_length_buf).unwrap(),
        Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Expected Header Length."))
    };

    let mut header_crc_buf = [0u8; 4];
    let (_, header_crc) = match file.read_exact(&mut header_crc_buf) {
        Ok(_) => le_u32(&mut header_crc_buf).unwrap(),
        Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Expected Header CRC."))
    };


    let mut header_bytes = vec![0u8; header_length as usize];
    file.read_exact(&mut header_bytes);

    if header_crc != crc32_rl::checksum(&header_bytes) {
        return Err(Error::new(ErrorKind::InvalidData, "Header CRC does not match header data."))
    }

    let header = header::get_header(&header_bytes).to_result().unwrap();

    println!("\nBody - Size: {}  |  CRC: {:?}", header_length, header_crc);
    println!("{:?}", header);


    let mut body_length_buf = [0; 4];
    let (_, body_length) = match file.read_exact(&mut body_length_buf) {
        Ok(_) => le_u32(&mut body_length_buf).unwrap(),
        Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Expected Body Length."))
    };

    let mut body_crc_buf = [0; 4];
    let (_, body_crc) = match file.read_exact(&mut body_crc_buf) {
        Ok(_) => le_u32(&mut body_crc_buf).unwrap(),
        Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Expected Body CRC."))
    };

    let mut body_bytes = vec![0u8; body_length as usize];
    file.read_exact(&mut body_bytes);


    if body_crc != crc32_rl::checksum(&body_bytes) {
        return Err(Error::new(ErrorKind::InvalidData, "Body CRC does not match body data."))
    }

    println!("\nBody - Size: {}  |  CRC: {:?}", body_length, body_crc);

    Ok(())
}



fn main() {
    parse(REPLAY_FILE_STR).unwrap();
}