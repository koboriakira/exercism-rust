use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let tester: AlphameticTester = input.parse().unwrap();
    println!("{:?}", tester.coefficients);
    println!("{:?}", tester.leading_variables);

    let variables: Vec<_> = tester.variables().collect();
    println!("{:?}", variables);

    let mut guess: HashMap<char, u8> = HashMap::new();
    for combination in (0..=9u8).permutations(variables.len()) {
        println!("{:?}", combination);
        for (i, d) in combination.iter().enumerate() {
            println!("{}, {}", i, d);
            guess.insert(*variables[i], *d);
        }
        if tester.test(&guess) {
            return Some(guess);
        }
    }
    None
}

struct AlphameticTester {
    coefficients: HashMap<char, i64>,
    leading_variables: HashSet<char>,
}

impl std::str::FromStr for AlphameticTester {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = match &input.split(" == ").collect::<Vec<_>>()[..] {
            &[lhs, rhs] => (lhs, rhs),
            _ => return Err(()),
        };

        let mut coefficients: HashMap<char, i64> = HashMap::new();
        let mut leading_variables: HashSet<char> = HashSet::new();
        for word in lhs.split(" + ") {
            if let Some(c) = word.chars().next() {
                leading_variables.insert(c);
            }
            for (i, c) in word.chars().rev().enumerate() {
                *coefficients.entry(c).or_insert(0) += 10i64.pow(i as u32);
            }
        }
        for word in rhs.split(" + ") {
            if let Some(c) = word.chars().next() {
                leading_variables.insert(c);
            }
            for (i, c) in word.chars().rev().enumerate() {
                *coefficients.entry(c).or_insert(0) -= 10i64.pow(i as u32);
            }
        }
        Ok(AlphameticTester {
            coefficients,
            leading_variables,
        })
    }
}

impl AlphameticTester {
    fn variables(&self) -> impl Iterator<Item = &char> {
        self.coefficients.keys()
    }

    fn test(&self, mapping: &HashMap<char, u8>) -> bool {
        if self
            .leading_variables
            .iter()
            .any(|c| mapping.get(c) == Some(&0))
        {
            return false;
        }
        self.coefficients.iter().fold(0, |sum, (c, coeff)| {
            println!(
                "sum: {}, c: {}, coeff :{}, mapping.get(c).unwrap(): {},acc: {}",
                sum,
                c,
                coeff,
                mapping.get(c).unwrap(),
                sum + *mapping.get(c).unwrap() as i64 * coeff
            );
            return sum + *mapping.get(c).unwrap() as i64 * coeff;
        }) == 0
    }
}
