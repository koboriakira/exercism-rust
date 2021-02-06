use rand::random;
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: random_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn reset_name(&mut self) {
        self.name = random_name();
    }
}

fn random_name() -> String {
    fn random_alphabet() -> char {
        (b'A' + random::<u8>().rem_euclid(26)) as char
    }

    fn random_number() -> char {
        (b'0' + random::<u8>().rem_euclid(10)) as char
    }

    (0..5)
        .map(|idx| match idx {
            0 | 1 => random_alphabet(),
            _ => random_number(),
        })
        .collect::<String>()
}
