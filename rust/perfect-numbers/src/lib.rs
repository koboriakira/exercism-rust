#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        1..=3 => Some(Classification::Deficient),
        num if num % 2 == 1 => {
            let sum = (1..num / 4 + 1)
                .map(|n| 2 * n + 1)
                .filter(|n| num % *n == 0)
                .sum::<u64>()
                + 1_u64;
            Some(check(sum, num))
        }
        num => {
            let sum = (2..num / 2 + 1).filter(|n| num % *n == 0).sum::<u64>() + 1_u64;
            Some(check(sum, num))
        }
    }
}

fn check(sum: u64, num: u64) -> Classification {
    match sum.cmp(&num) {
        std::cmp::Ordering::Less => Classification::Deficient,
        std::cmp::Ordering::Equal => Classification::Perfect,
        std::cmp::Ordering::Greater => Classification::Abundant,
    }
}
