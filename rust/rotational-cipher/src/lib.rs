pub fn rotate(input: &str, key: i8) -> String {
    let key = key.rem_euclid(26) as u8;
    input
        .chars()
        .map(|c| char_rotate(c, key))
        .collect::<String>()
}

fn char_rotate(c: char, key: u8) -> char {
    if !c.is_alphabetic() {
        return c;
    }

    let base_byte = if c.is_lowercase() { b'a' } else { b'A' };
    (((c as u8) - base_byte + key).rem_euclid(26) + base_byte) as char
}
