/// ?.......?
/// .?.....?.?
/// ..?...?...?
/// ...?.?.....?
/// ....?.......?

pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let init: Vec<Vec<char>> = vec![];
        let result = text.chars().fold(
            (init, 0 as usize, 1 as isize),
            |(mut acc, idx, next), ch| {
                match acc.get_mut(idx) {
                    Some(char_list) => char_list.push(ch),
                    None => acc.push(vec![ch]),
                }
                if idx == 0 && next == -1 {
                    (acc, 1, 1)
                } else if (idx as isize + next) as usize == self.rails {
                    (acc, idx - 1, -1)
                } else {
                    (acc, (idx as isize + next) as usize, next)
                }
            },
        );
        println!("{:?}", result.0);

        result.0.into_iter().fold(String::new(), |mut acc, result| {
            acc.push_str(&result.into_iter().collect::<String>());
            acc
        })
    }

    pub fn decode(&self, cipher: &str) -> String {
        unimplemented!("Decode this ciphertext: {}", cipher)
    }
}
