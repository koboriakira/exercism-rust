use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    parallel(divide_input(input, worker_count))
}

fn divide_input<'a>(input: &'a [&str], worker_count: usize) -> Vec<Vec<&'a str>> {
    let mut inputs = (0..worker_count)
        .map(|_| vec![])
        .collect::<Vec<Vec<&str>>>();
    input.iter().enumerate().for_each(|(i, _str)| {
        inputs[i % worker_count].push(_str);
    });
    inputs
}

fn parallel(inputs: Vec<Vec<&str>>) -> HashMap<char, usize> {
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

fn analyse(input: Vec<&str>) -> HashMap<char, usize> {
    input
        .into_iter()
        .flat_map(|el| el.chars())
        .filter(|c| c.is_alphabetic())
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c.to_ascii_lowercase()).or_insert(0 as usize) += 1 as usize;
            map
        })
}
