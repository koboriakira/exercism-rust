use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input_chunks: Vec<&[&str]> = input.chunks(worker_count).collect();
    parallel(input_chunks)
}

fn parallel(inputs: Vec<&[&str]>) -> HashMap<char, usize> {
    inputs
        .into_iter()
        .map(|input| crossbeam::scope(|_| analyse(input)))
        .fold(HashMap::new(), |mut map, res| {
            res.ok().unwrap().into_iter().for_each(|(c, size)| {
                *map.entry(c).or_insert(0) += size;
            });
            map
        })
}

fn analyse(input: &[&str]) -> HashMap<char, usize> {
    input
        .into_iter()
        .flat_map(|el| el.chars())
        .filter(|c| c.is_alphabetic())
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c.to_ascii_lowercase()).or_insert(0 as usize) += 1 as usize;
            map
        })
}
