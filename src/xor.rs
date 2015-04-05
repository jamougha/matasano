pub fn encode<I, J>(a: I, b: J) -> Vec<u8>
    where I: Iterator<Item = u8>, J: Iterator<Item = u8>
{
    a.zip(b).map(|(ab, bb)| ab ^ bb).collect()
}