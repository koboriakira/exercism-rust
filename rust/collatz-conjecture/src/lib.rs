pub fn collatz(n: u64) -> Option<u64> {
    collatz_recurcively(n, 0)
}

fn collatz_recurcively(n: u64, cnt: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(cnt),
        n => match n % 2 == 0 {
            true => collatz_recurcively(n / 2, cnt + 1),
            _ => collatz_recurcively(3 * n + 1, cnt + 1),
        },
    }
}
