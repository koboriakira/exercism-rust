use std::collections::HashMap;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn make_dictionary(
    a: &i32,
    b: &i32,
    inverse: bool,
) -> Result<HashMap<char, char>, AffineCipherError> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    (0..26).for_each(|x| {
        let calculated = (a * x + b) % 26;
        map.entry(calculated).or_insert(x);
    });
    match map.len() {
        26 => Ok(map
            .iter()
            .map(|(k, v)| ((*v as u8) + 97, (*k as u8) + 97))
            .map(|(b1, b2)| match inverse {
                true => (b2 as char, b1 as char),
                false => (b1 as char, b2 as char),
            })
            .collect()),
        _ => Err(AffineCipherError::NotCoprime(*a)),
    }
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    make_dictionary(&a, &b, false).and_then(|dictionary| {
        Ok(plaintext
            .to_ascii_lowercase()
            .chars()
            .filter_map(|c| convert(c, &dictionary))
            .collect::<Vec<_>>()
            .chunks(5)
            .map(|chunks| chunks.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(" "))
    })
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    make_dictionary(&a, &b, true).and_then(|dictionary| {
        Ok(ciphertext
            .chars()
            .filter_map(|c| convert(c, &dictionary))
            .collect::<String>())
    })
}

fn convert(c: char, dictionary: &HashMap<char, char>) -> Option<char> {
    match (c.is_ascii_alphabetic(), c.is_numeric()) {
        (true, _) => Some(*dictionary.get(&c).unwrap()),
        (_, true) => Some(c),
        (_, _) => None,
    }
}
