pub struct Bytes<I> where I: Iterator<Item = u8>{
    iter: I,
    next_byte: Option<u8>,
    next_bit: u8,
    byte_size: u8,
    in_byte_size: u8,
}

impl<I> Iterator for Bytes<I> where I: Iterator<Item = u8> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let mut out_byte = 0;
        let mut out_bit = 1 << (self.byte_size - 1);
        loop {
            if out_bit == 0 {
                return Some(out_byte);
            }

            if self.next_bit == 0 {
                self.next_byte = self.iter.next();
                self.next_bit = 1 << (self.in_byte_size - 1);
            }

            if let Some(next_byte) = self.next_byte {
                if self.next_bit & next_byte != 0 {
                    out_byte |= out_bit;
                }
            } else {
                return None;
            }

            out_bit >>= 1;
            self.next_bit >>= 1;
        }
    }
}

pub trait Packable<I: Iterator> {
    fn unpack(self, bits: u8) -> Bytes<I>;
    fn pack(self, bits: u8) -> Bytes<I>;
}

impl<I> Packable<I> for I where I: Iterator<Item = u8> {
    fn unpack(self, byte_size: u8) -> Bytes<I> {
        Bytes {
            iter: self,
            next_byte: None,
            next_bit: 0,
            byte_size: byte_size,
            in_byte_size: 8,
        }
    }

    fn pack(self, byte_size: u8) -> Bytes<I> {
        Bytes {
            iter: self,
            next_byte: None,
            next_bit: 0,
            byte_size: 8,
            in_byte_size: byte_size,
        }
    }
}

pub fn from_hex(hex: &[u8]) -> Vec<u8> {
    hex.iter().map(|x| hex_digit_to_num(*x)).pack(4).collect()
}

pub fn to_hex(hex: &[u8]) -> Vec<u8> {
    hex.iter().map(|x| *x).unpack(4).map(num_to_hex_digit).collect()
}

pub fn hex_digit_to_num(digit: u8) -> u8 {
    match digit as char {
        '0'...'9' => digit - '0' as u8,
        'A'...'F' => 0xA + digit - 'A' as u8,
        'a'...'f' => 0xA + digit - 'a' as u8,
        _ => panic!("{} is not a hexadecimal digit", digit as char)
    }
}

pub fn num_to_hex_digit(num: u8) -> u8 {
    match num {
        0...9 => (num + ('0' as u8)),
        10...15 => (num - 0xA + ('a' as u8)),
        _ => panic!("{} cannot be converted to hexadecimal", num),
    }
}

pub fn to_mime_base64(digit: u8) -> u8 {
    match digit {
        0...25 => 'A' as u8 + digit,
        26...51 => 'a' as u8 + digit - 26,
        52...61 => '0' as u8 + digit - 52,
        62 => '+' as u8,
        63 => '/' as u8,
        _ => panic!("{} is not a base 64 digit")
    }
}
