pub fn abbreviate(phrase: &str) -> String {
    fn get_abbreviate(word: &str) -> Vec<char> {
        if word.is_empty() {
            return vec![];
        };
        let splited_words = word.split("-").collect::<Vec<&str>>();
        if splited_words.len() > 1 {
            return splited_words
                .iter()
                .map(|w| {
                    if let Some(first_char) = w.chars().nth(0) {
                        first_char.to_ascii_uppercase()
                    } else {
                        ' '
                    }
                })
                .filter(|c| !c.eq(&' '))
                .collect();
        }
        if !word.chars().any(|c| c.is_ascii_lowercase()) {
            return vec![word.chars().nth(0).unwrap()];
        }
        word.chars()
            .filter(|c| !['_', '-'].contains(&c))
            .enumerate()
            .filter(|(i, c)| match (i, c.is_ascii_uppercase()) {
                (0, _) => true,
                (_, true) => true,
                (_, false) => false,
            })
            .map(|(_, c)| c.to_ascii_uppercase())
            .collect()
    }

    match phrase.is_empty() {
        true => String::new(),
        false => phrase.split(" ").map(get_abbreviate).flatten().collect(),
    }
}
