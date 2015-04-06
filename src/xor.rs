use std::iter::repeat;

pub fn encode<I, J>(a: I, b: J) -> Vec<u8>
    where I: Iterator<Item = u8>, J: Iterator<Item = u8>
{
    a.zip(b).map(|(ab, bb)| ab ^ bb).collect()
}

pub fn best_decoding(ciphertext: &[u8]) -> (u32, String) {

    let mut best_key = 0u8;
    let mut best_key_score = 0u32;

    for key_byte in 0.. {
        let mut counts = [0u32; 256];
        let key = repeat(key_byte);
        let plaintext = encode(key, ciphertext.iter().map(|b| *b));
        for b in plaintext {
            counts[b as usize] += 1;
        }

        let score = "etaoin shrudlu".chars().fold(0, |score, c| score + counts[c as usize]);
        if score > best_key_score {
            best_key = key_byte;
            best_key_score = score;
        }

    }

    let plaintext = encode(repeat(best_key), ciphertext.iter().map(|b| *b));

    (best_key_score, plaintext.iter().map(|x| *x as char).collect::<String>())

}