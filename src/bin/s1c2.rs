extern crate matasano;
use matasano::bases::{from_hex, to_hex};
use matasano::xor::encode;
use std::str::from_utf8;

fn main() {
    let plaintext = from_hex(b"1c0111001f010100061a024b53535009181c");
    let key = from_hex(b"686974207468652062756c6c277320657965");
    let ciphertext = "746865206b696420646f6e277420706c6179";

    let ct = to_hex(&encode(plaintext.into_iter(), key.into_iter()));

    println!("{:?}", from_utf8(&ct).unwrap());
    assert_eq!(ciphertext, from_utf8(&ct).unwrap());
}