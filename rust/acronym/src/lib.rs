pub fn abbreviate(phrase: &str) -> String {
    fn get_abbreviate(word: &str) -> String {
        let splited_words = word.split("-").collect::<Vec<&str>>();
        if splited_words.len() > 1 {
            return splited_words
                .iter()
                .map(|&w| abbreviate(w))
                .collect::<Vec<String>>()
                .join("");
        }
        if !word.chars().any(|c| c.is_ascii_lowercase()) {
            return word.chars().nth(0).unwrap().to_string();
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
        false => phrase
            .split(" ")
            .map(get_abbreviate)
            .collect::<Vec<String>>()
            .join(""),
    }
}
