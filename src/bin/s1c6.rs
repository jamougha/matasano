fn main() {
	let a = b"this is a test";
	let b = b"wokka wokka!!!";
	let mut sum = 0;
	for (c, d) in a.iter().zip(b.iter()) {
		let mut x = c ^ d;
		while x != 0 {
			sum += x & 1;
			x >>= 1;
		}
	}

	println!("{:?}", sum);
}