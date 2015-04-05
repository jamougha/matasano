extern crate matasano;

use matasano::bases::{from_hex, to_hex};
use matasano::xor::encode;
use std::iter::repeat;
use std::str::from_utf8;

fn main() {
    let ciphertext = from_hex(b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let mut best_key = 0u8;
    let mut best_key_score = 0u32;

    for key_byte in 0..255 {
        let mut counts = [0u8; 256];
        let key = repeat(key_byte);
        let plaintext = encode(key, ciphertext.iter().map(|x| *x));
        for b in plaintext {
            counts[b as usize] += 1;
        }

        let num_letters = counts['A' as usize..'z' as usize].iter().fold(0, |a, b| a + (*b as u32));
        if num_letters > best_key_score {
            best_key = key_byte;
            best_key_score = num_letters;
        }

    }

    let plaintext = encode(repeat(best_key), ciphertext.iter().map(|x| *x));

    println!("{:?}", from_utf8(&plaintext));
}