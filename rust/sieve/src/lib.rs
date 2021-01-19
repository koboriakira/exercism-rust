pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (1..upper_bound + 1)
        .into_iter()
        .filter(|i| is_prime(i))
        .collect()
}

fn is_prime(num: &u64) -> bool {
    match (num, num % 2) {
        (1, _) => false,
        (2, _) => true,
        (_, 0) => false,
        (_, _) => {
            let max = (*num as f64).sqrt() as u64;
            let mut division = 3u64;
            while division <= max {
                match num % division {
                    0 => return false,
                    _ => division += 2,
                }
            }
            true
        }
    }
}
