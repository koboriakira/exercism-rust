use rand::Rng;
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
        (b'A' + rand::thread_rng().gen_range(0..26) as u8) as char
    }

    fn random_number() -> String {
        format!("{:03}", rand::thread_rng().gen_range(0..1000))
    }

    format!(
        "{}{}{}",
        random_alphabet(),
        random_alphabet(),
        random_number()
    )
}
