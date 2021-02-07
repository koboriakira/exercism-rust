use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    convert(key, s, sub_encode)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    convert(key, s, sub_decode)
}

fn convert(key: &str, s: &str, convert_function: fn(char, char) -> char) -> Option<String> {
    validate(key, s)
        .and_then(|(key, s)| {
            s.chars()
                .zip(key.chars().cycle())
                .try_fold(String::new(), |mut acc, (c, key)| {
                    if !is_lower_alphabet(c) || !is_lower_alphabet(key) {
                        return Err(());
                    }
                    acc.push(convert_function(c, key));
                    Ok(acc)
                })
        })
        .ok()
}

fn is_lower_alphabet(c: char) -> bool {
    c.is_ascii_alphabetic() && c.is_lowercase()
}

fn validate<'a>(key: &'a str, s: &'a str) -> Result<(&'a str, &'a str), ()> {
    if key.is_empty() || s.is_empty() {
        return Err(());
    }
    Ok((key, s))
}

pub fn encode_random(s: &str) -> (String, String) {
    fn random_key() -> char {
        (rand::thread_rng().gen_range(b'a', b'z' + 1) as u8) as char
    }
    fn generate_key() -> String {
        (0..100).fold(String::new(), |mut acc, _| {
            acc.push(random_key());
            acc
        })
    }
    let key = generate_key();
    (key.clone(), encode(&key[..], s).unwrap())
}

fn sub_encode(alphabet: char, key: char) -> char {
    let alphabet = (alphabet as u8) - b'a';
    let key = (key as u8) - b'a';
    let res = (alphabet + key).rem_euclid(26) + 97;
    res as char
}

fn sub_decode(alphabet: char, key: char) -> char {
    let alphabet = (alphabet as u8) - b'a';
    let key = (key as u8) - b'a';
    let res = (alphabet as i8 - key as i8).rem_euclid(26) + 97;
    (res as u8) as char
}
