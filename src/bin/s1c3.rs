extern crate matasano;

use matasano::bases::{from_hex};
use matasano::xor::best_decoding;

fn main() {
    let ciphertext = from_hex(b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    println!("{:?}", best_decoding(&ciphertext));
}