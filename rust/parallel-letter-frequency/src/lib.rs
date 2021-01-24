use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    input.iter().for_each(|i| {
        sub_frequency(i).iter().for_each(|(c, size)| {
            *result.entry(*c).or_insert(0) += size;
        })
    });
    result
}

fn sub_frequency(input: &str) -> HashMap<char, usize> {
    let result: HashMap<char, usize> = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0 as usize) += 1 as usize;
            map
        });
    result
}
