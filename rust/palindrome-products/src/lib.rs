use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    pairs: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            pairs: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        unimplemented!("return the value of this palindrome")
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.pairs.push((a, b));
    }

    pub fn sort(&mut self) {
        self.pairs.sort();
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let (mut min_palindrome_result, mut max_palindrome_result) = (u64::MAX, u64::MIN);
    let (mut min_palindrome, mut max_palindrome) = (
        Palindrome::new(u64::MAX, u64::MAX),
        Palindrome::new(u64::MIN, u64::MIN),
    );

    for i in min..max + 1 {
        if is_palindrome(&i.pow(2)) {
            match i.pow(2).cmp(&min_palindrome_result) {
                Ordering::Less => {
                    min_palindrome = Palindrome::new(i, i);
                    min_palindrome_result = i.pow(2)
                }
                Ordering::Equal => min_palindrome.insert(i, i),
                Ordering::Greater => {}
            }
            match i.pow(2).cmp(&max_palindrome_result) {
                Ordering::Less => {}
                Ordering::Equal => max_palindrome.insert(i, i),
                Ordering::Greater => {
                    max_palindrome = Palindrome::new(i, i);
                    max_palindrome_result = i.pow(2)
                }
            }
        }
    }
    for (a, b) in (min..max + 1).tuple_combinations() {
        if is_palindrome(&(a * b)) {
            match (a * b).cmp(&min_palindrome_result) {
                Ordering::Less => {
                    min_palindrome = Palindrome::new(a, b);
                    min_palindrome_result = a * b
                }
                Ordering::Equal => min_palindrome.insert(a, b),
                Ordering::Greater => {}
            }
            match (a * b).cmp(&max_palindrome_result) {
                Ordering::Less => {}
                Ordering::Equal => max_palindrome.insert(a, b),
                Ordering::Greater => {
                    max_palindrome = Palindrome::new(a, b);
                    max_palindrome_result = a * b
                }
            }
        }
    }
    if min_palindrome.eq(&Palindrome::new(u64::MAX, u64::MAX)) {
        None
    } else {
        min_palindrome.sort();
        max_palindrome.sort();
        Some((min_palindrome, max_palindrome))
    }
}

fn is_palindrome(num: &u64) -> bool {
    if num % 10 == 0 {
        return false;
    }
    num.to_string()
        .eq(&num.to_string().chars().rev().collect::<String>())
}
