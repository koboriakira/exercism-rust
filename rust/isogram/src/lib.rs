use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    match candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>()
    {
        x if x.len() == 0 => true,
        x => x.len() == x.into_iter().collect::<HashSet<char>>().len(),
    }
}
