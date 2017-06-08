use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("F32599A54B1831A58C6C55A5334890AF.replay").unwrap();

    let mut buffer = Vec::new();

    let size = file.read_to_end(&mut buffer).unwrap();

    println!("Size of file: {}kb.", size/1024);
}