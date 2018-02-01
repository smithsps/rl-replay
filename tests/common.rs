#[macro_use]
extern crate nom;
extern crate rl_replays;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use nom::le_u32;

use rl_replays::{header, body, crc32_rl};

//const DEFAULT_REPLAY_BUFFER: usize = 2 * 1024 * 1024;


fn parse(file: &str) -> io::Result<()> {
    let mut file = BufReader::new(File::open(file)?);

    let header_length = read_u32(&mut file)?;
    let header_crc = read_u32(&mut file)?;

    let mut header_bytes = vec![0u8; header_length as usize];
    file.read_exact(&mut header_bytes)?;

    if header_crc != crc32_rl::checksum(&header_bytes) {
        return Err(Error::new(ErrorKind::InvalidData, "Header CRC does not match header data."))
    }

    let header = header::get_header(&header_bytes).to_result().unwrap();
    println!("\nBody - Size: {}  |  CRC: {:08X}", header_length, header_crc);
    println!("{:?}", header);


    let body_length = read_u32(&mut file)?;
    let body_crc = read_u32(&mut file)?;

    let mut body_bytes = vec![0u8; body_length as usize];
    file.read_exact(&mut body_bytes)?;

    if body_crc != crc32_rl::checksum(&body_bytes) {
        return Err(Error::new(ErrorKind::InvalidData, "Body CRC does not match body data."))
    }

    let _body = body::get_body(&body_bytes).to_result().unwrap();

    println!("\nBody - Size: {}  |  CRC: {:08X}", body_length, body_crc);
    //println!("{:?}", body);

    Ok(())
}

fn read_u32(file: &mut BufReader<File>) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    match file.read_exact(&mut buf) {
        Ok(_) => Ok(le_u32(&mut buf).unwrap().1),
        Err(x) => Err(x)
    }
}

#[test]
fn integratation_test() {
    //static REPLAY_FILE_STR: &'static str = "replays/7560D3FE446244A56C0EB198007F2B92.replay";
    static REPLAY_FILE_STR: &'static str = "replays/1EBA9EA845DB4BD7809E78A7F4A7F1EC.replay";
    //static REPLAY_FILE_STR: &'static str = "replays/F32599A54B1831A58C6C55A5334890AF.replay";

    parse(REPLAY_FILE_STR).unwrap();
}