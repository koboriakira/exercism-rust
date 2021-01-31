/// Compute the Scrabble score for a word.

pub fn score(word: &str) -> u64 {
    word.chars()
        .filter_map(|c| Some(c.to_ascii_uppercase()))
        .fold(0_u64, |acc, c| acc + get_letter_score(c))
}

fn get_letter_score(letter: char) -> u64 {
    match letter {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0,
    }
}
