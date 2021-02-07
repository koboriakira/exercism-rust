use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    validate(key, s)
        .and_then(|(key, s)| {
            let mut keys: Vec<char> = key.chars().rev().collect();
            Ok(s.chars()
                .map(|c| match keys.pop() {
                    Some(key) => sub_encode((c, key)),
                    _ => c,
                })
                .collect::<String>())
        })
        .ok()
}

fn validate<'a>(key: &'a str, s: &'a str) -> Result<(&'a str, &'a str), ()> {
    if key.is_empty() || s.is_empty() {
        return Err(());
    }
    if !key
        .chars()
        .chain(s.chars())
        .all(|c| c.is_ascii_alphabetic() && c.is_lowercase())
    {
        return Err(());
    }
    Ok((key, s))
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    validate(key, s)
        .and_then(|(key, s)| {
            let mut keys: Vec<char> = key.chars().rev().collect();
            Ok(s.chars()
                .map(|c| match keys.pop() {
                    Some(key) => sub_decode((c, key)),
                    _ => c,
                })
                .collect::<String>())
        })
        .ok()
}

pub fn encode_random(s: &str) -> (String, String) {
    fn random_key() -> char {
        (rand::thread_rng().gen_range(b'a', b'z' + 1) as u8) as char
    }
    fn generate_key() -> String {
        let mut keys = vec![];
        while keys.len() < 100 {
            keys.push(random_key());
        }
        keys.into_iter().collect()
    }
    let key = generate_key();
    (key.clone(), encode(&key[..], s).unwrap())
}

fn sub_encode((alphabet, key): (char, char)) -> char {
    let alphabet = (alphabet as u8) - b'a';
    let key = (key as u8) - b'a';
    let res = (alphabet + key).rem_euclid(26) + 97;
    res as char
}

fn sub_decode((alphabet, key): (char, char)) -> char {
    let alphabet = (alphabet as u8) - b'a';
    let key = (key as u8) - b'a';
    let res = (alphabet as i8 - key as i8).rem_euclid(26) + 97;
    (res as u8) as char
}
