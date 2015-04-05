extern crate matasano;
use matasano::bases::{Packable, hex_digit_to_num, to_mime_base64};

fn main() {
	// 49 = 32 + 16 + 1 = 00110001
	// ABCDEFGHIJKLMNOPQRS
	let hex  = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    assert_eq!(base64, hex.iter().map(|x| *x)
                          .unpack(8)
                          .map(hex_digit_to_num)
                          .pack(4)
                          .unpack(6)
                          .map(to_mime_base64)
                          .map(|b| b as char)
                          .collect::<String>());
	//assert_eq!(base64, from_utf8(&hex.hex_to_base64()[..]).unwrap());
}

