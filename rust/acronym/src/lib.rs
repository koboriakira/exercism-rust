// pub fn abbreviate(phrase: &str) -> String {
//     fn get_abbreviate(word: &str) -> String {
//         let splited_words = word.split("-").collect::<Vec<&str>>();
//         if splited_words.len() > 1 {
//             return splited_words
//                 .iter()
//                 .map(|&w| abbreviate(w))
//                 .collect::<Vec<String>>()
//                 .join("");
//         }
//         if !word.chars().any(|c| c.is_ascii_lowercase()) {
//             return word.chars().nth(0).unwrap().to_string();
//         }
//         word.chars()
//             .filter(|c| !['_', '-'].contains(&c))
//             .enumerate()
//             .filter(|(i, c)| match (i, c.is_ascii_uppercase()) {
//                 (0, _) => true,
//                 (_, true) => true,
//                 (_, false) => false,
//             })
//             .map(|(_, c)| c.to_ascii_uppercase())
//             .collect()
//     }

//     match phrase.is_empty() {
//         true => String::new(),
//         false => phrase
//             .split(" ")
//             .map(get_abbreviate)
//             .collect::<Vec<String>>()
//             .join(""),
//     }
// }

pub fn abbreviate(_phrase: &str) -> String {
    fn is_abbreviate(pair: &&[char]) -> bool {
        let first_char = pair[0];
        let second_char = pair[1];
        ([' ', '-'].contains(&first_char) && second_char.is_alphanumeric())
            || (!first_char.is_ascii_uppercase() && second_char.is_ascii_uppercase())
    }

    let mut phrase = String::from(" ");
    phrase.push_str(_phrase);

    phrase
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .filter(is_abbreviate)
        .map(|word| word[1])
        .collect::<String>()
        .to_ascii_uppercase()
}
