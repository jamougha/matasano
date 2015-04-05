pub mod base64 {

    pub struct Bytes<'a, I: 'a> {
        iter: &'a mut I,
        next_byte: Option<&'a u8>,
        next_bit: u8,
        byte_size: u8,
        out_byte: u8,
    }

    impl<'a, 'b, I: Iterator<Item = &'a u8>> Iterator for Bytes<'a, I> {
        type Item = &'b u8;

        fn next(&'b mut self) -> Option<&'b u8> {
            self.out_byte = 0;
            let mut out_bit = 1 << (self.byte_size - 1);
            loop {
                if out_bit == 0 {
                    return Some(&self.out_byte);
                }
                let next_bit = self.next_bit;
                if next_bit == 0 {
                    self.next_byte = self.iter.next();
                    self.next_bit = 1 << 7;
                }

                if let Some(&next_byte) = self.next_byte {
                    if self.next_bit & next_byte != 0 {
                        self.out_byte |= out_bit;
                    }
                } else {
                    return None;
                }

                out_bit >>= 1;
                self.next_bit >>= 1;
            }
        }
    }

    pub trait Packable<'a, I: Iterator> {
        fn unpack(&'a mut self, bits: u8) -> Bytes<'a, I>;
    }

    // impl<'a, I> Packable<'a, I> for I where I: Iterator<Item = u8> {
    //     fn unpack(&'a mut self, byte_size: u8) -> Bytes<'a, I> {
    //         Bytes {
    //             iter: self,
    //             next_byte: None,
    //             next_bit: 0,
    //             byte_size: byte_size,
    //         }
    //     }
    // }

    impl<'a, I> Packable<'a, I> for I where I: Iterator<Item = &'a u8> {
        fn unpack(&'a mut self, byte_size: u8) -> Bytes<'a, I> {
            Bytes {
                iter: self,
                next_byte: None,
                next_bit: 0,
                byte_size: byte_size,
                out_byte: 0,
            }
        }
    }

    pub fn hex_digit_to_num(digit: u8) -> u8 {
        match digit as char {
            '0'...'9' => digit - '0' as u8,
            'A'...'F' => 0xA + digit - 'A' as u8,
            'a'...'f' => 0xA + digit - 'a' as u8,
            _ => panic!("{} is not a hexadecimal digit", digit as char)
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

}
