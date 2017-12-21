#[macro_use]
extern crate nom;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use nom::{le_u32};

mod header;
mod crc32_rl;

static REPLAY_FILE_STR: &'static str = "replays/1EBA9EA845DB4BD7809E78A7F4A7F1EC.replay";
//static REPLAY_FILE_STR: &'static str = "replays/F32599A54B1831A58C6C55A5334890AF.replay";
const DEFAULT_REPLAY_BUFFER: usize = 5400;


fn parse(file: &str) -> io::Result<()> {
    let mut file = File::open(file)?;
    

    let mut hlen_buf = [0; 4];
    file.read_exact(&mut hlen_buf);
    let (_, header_length) = le_u32(&mut hlen_buf).unwrap();
    
    //header_length += 4; // CRC is not initially included. 
    //https://github.com/gcc-mirror/gcc/blob/master/libiberty/crc32.c
    let mut crc_buf = [0; 4];
    file.read_exact(&mut crc_buf);
    let (_, crc) = le_u32(&mut crc_buf).unwrap();


    println!("Header Length: {:?}\n", header_length);

    let mut h_buf = BufReader::with_capacity(header_length as usize, file);
    let h_bytes = h_buf.fill_buf()?;

    print!("{:?}\n", crc32_rl::checksum(h_bytes));
    print!("{:?}\n", crc);

    let header = header::get_header(h_bytes).to_result().unwrap();
    println!("{:?}", header);

    Ok(())
}



fn main() {
    parse(REPLAY_FILE_STR).unwrap();
}