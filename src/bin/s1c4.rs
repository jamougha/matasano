extern crate matasano;

use matasano::xor::best_decoding;
use matasano::bases::from_hex;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open(Path::new("/home/jamougha/Downloads/matasano1.4.txt")).unwrap();
    let reader = BufReader::new(file);
    let (_, plaintext) = reader.lines().filter_map(|line| {
        line.ok().map(|line|
            best_decoding(&from_hex((&*line).as_bytes())))
    }).fold((0u32, "".to_string()), |best, next|
        if next.0 > best.0 { next } else { best }
    );

    println!("{:?}", plaintext);
}