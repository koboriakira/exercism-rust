use crossbeam;
use std::{collections::HashMap, sync};
use sync::Arc;
use sync::Mutex;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    parallel(input, worker_count)
    // single(input)
}

fn single(input: &[&str]) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    input.iter().for_each(|i| {
        sub_frequency(i).iter().for_each(|(c, size)| {
            *result.entry(*c).or_insert(0) += size;
        })
    });
    result
}

fn parallel(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let result: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let input = Arc::new(Mutex::new(input.clone().iter()));
    let _ = crossbeam::scope(|scope| {
        for _ in 0..worker_count {
            let result = Arc::clone(&result);
            let input = Arc::clone(&input);
            scope.spawn(move |_| {
                while let Some(el) = input.lock().unwrap().next() {
                    println!("{}", el);
                    sub_frequency(el).iter().for_each(|(c, size)| {
                        *result.lock().unwrap().entry(*c).or_insert(0) += size;
                        // *result.entry(*c).or_insert(0) += size;
                    });
                }
            });
        }
    });
    let result = result
        .lock()
        .unwrap()
        .iter()
        .map(|(c, u)| (*c, *u))
        .collect::<HashMap<char, usize>>();
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
