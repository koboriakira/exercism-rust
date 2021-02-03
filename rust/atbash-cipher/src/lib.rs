/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_ascii_lowercase()
        .chars()
        .filter_map(convert)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|char_chunk| char_chunk.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .to_ascii_lowercase()
        .chars()
        .filter_map(convert)
        .collect::<String>()
}

fn convert(c: char) -> Option<char> {
    match (c.is_ascii_alphabetic(), c.is_numeric()) {
        (true, _) => Some((122 - (c as u8) + 97) as char),
        (_, true) => Some(c),
        (_, _) => None,
    }
}
