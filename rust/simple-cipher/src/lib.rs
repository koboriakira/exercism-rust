use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    let mut keys = key.chars();
    let mut characters: Vec<char> = s.chars().rev().collect();

    let mut result: Vec<char> = vec![];
    while let Some(key) = keys.next() {
        if !key.is_ascii_alphabetic() || key.is_uppercase() {
            return None;
        }
        if let Some(character) = characters.pop() {
            if !character.is_ascii_alphabetic() || character.is_uppercase() {
                return None;
            }
            println!("char: {}  key: {}", character, key);
            result.push(sub_encode((character, key)));
        } else {
            break;
        }
    }

    if characters.len() > 0 {
        result.append(&mut characters.into_iter().rev().collect::<Vec<char>>());
    }

    let encoded_string = result.into_iter().collect::<String>();
    println!("{}", encoded_string);
    Some(encoded_string)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    let mut keys = key.chars();
    let mut characters: Vec<char> = s.chars().rev().collect();

    let mut result: Vec<char> = vec![];
    while let Some(key) = keys.next() {
        if !key.is_ascii_alphabetic() || key.is_uppercase() {
            return None;
        }
        if let Some(character) = characters.pop() {
            if !character.is_ascii_alphabetic() || character.is_uppercase() {
                return None;
            }
            println!("char: {}  key: {}", character, key);
            result.push(sub_decode((character, key)));
        } else {
            break;
        }
    }

    if characters.len() > 0 {
        result.append(&mut characters.into_iter().rev().collect::<Vec<char>>());
    }

    let encoded_string = result.into_iter().collect::<String>();
    println!("{}", encoded_string);
    Some(encoded_string)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut keys = vec![];
    while keys.len() < 100 {
        keys.push(random_key());
    }

    let key = keys.into_iter().collect::<String>();

    (key.clone(), encode(&key[..], s).unwrap())
}

fn random_key() -> char {
    (rand::thread_rng().gen_range(b'a', b'z' + 1) as u8) as char
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
