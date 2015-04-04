pub mod base64 {
	pub trait Conversions {

		fn to_hex(&self) -> Vec<u8>;
		fn hex_to_bits(&self) -> Vec<u8>;

		fn hex_to_base64(&self) -> Vec<u8> {
			self.hex_to_bits()
				.chunks(6)
				.map(|chunk| {
					let b64 = (0..6).zip(chunk.iter())
					      		    .map(|(i, bit)| bit << (5 - i))
					      		    .fold(0, |x, y| x + y);
			        to_mime_base64(b64)
				}).collect()
		}
	}

	fn hex_digit_to_num(digit: u8) -> u8 {
		match digit as char {
			'0'...'9' => digit - '0' as u8,
			'A'...'F' => 0xA + digit - 'A' as u8,
			'a'...'f' => 0xA + digit - 'a' as u8,
			_ => panic!("{} is not a hexadecimal digit", digit as char)
		}
	}

	fn to_mime_base64(digit: u8) -> u8 {
		match digit {
			0...25 => 'A' as u8 + digit,
			26...51 => 'a' as u8 + digit - 26,
			52...61 => '0' as u8 + digit - 52,
			62 => '+' as u8,
			63 => '/' as u8,
			_ => panic!("{} is not a base 64 digit")
		}
	}

	impl Conversions for [u8] {

		fn hex_to_bits(&self) -> Vec<u8> {
			self.iter().flat_map(|&digit| {
				let nibble = hex_digit_to_num(digit);
				(0..4).map(move |i| (nibble >> (3 - i)) & 1 as u8)
			}).collect::<Vec<u8>>()
		}

		fn to_hex(&self) -> Vec<u8> {
			Vec::new()
		}

	}
}

mod xor {
	fn encode(key: &[u8], text: &[u8]) -> Vec<u8> {
		key.iter()
		   .zip(text.iter())
		   .map(|(k, t)| k ^ t)
		   .collect()
	}
}