pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string(); digits.len() + 1],
        len => digits
            .chars()
            .collect::<Vec<char>>()
            .windows(len)
            .map(|window| window.into_iter().collect::<String>())
            .collect(),
    }
}
