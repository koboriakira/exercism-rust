pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let allergies = convert_base10_to_base2(score).and_then(|score| {
            println!("{:?}", score);
            Ok(score
                .into_iter()
                .rev()
                .enumerate()
                .filter_map(|(i, s)| {
                    println!("{}, {}", i, s);
                    let allergen = match i {
                        0 => Allergen::Eggs,
                        1 => Allergen::Peanuts,
                        2 => Allergen::Shellfish,
                        3 => Allergen::Strawberries,
                        4 => Allergen::Tomatoes,
                        5 => Allergen::Chocolate,
                        6 => Allergen::Pollen,
                        7 => Allergen::Cats,
                        _ => return None,
                    };
                    match s {
                        0 => None,
                        _ => Some(allergen),
                    }
                })
                .collect::<Vec<Allergen>>())
        });
        println!("{:?}", allergies);
        Self {
            allergies: allergies.ok().unwrap(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn convert_base10_to_base2(number: u32) -> Result<Vec<u32>, Error> {
    fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
        let max_num = number.into_iter().max();
        match (max_num, from_base, to_base) {
            (_, 0..=1, _) => return Err(Error::InvalidInputBase),
            (_, _, 0..=1) => return Err(Error::InvalidOutputBase),
            (Some(max_num), _, _) if max_num >= &from_base => {
                return Err(Error::InvalidDigit(from_base))
            }
            (_, _, _) => {
                let total = number
                    .into_iter()
                    .rev()
                    .enumerate()
                    .map(|(i, n)| n * from_base.pow(i as u32))
                    .sum::<u32>();
                println!("total: {}", total);

                let digits = calculate_digit(total, to_base);
                println!("digits: {}", digits);

                let result =
                    (0..digits)
                        .rev()
                        .fold((total, vec![]), |(rest, mut result), digit| {
                            let multiply = calculate_multiply(rest, to_base, digit as u32);
                            result.push(multiply);
                            (rest - to_base.pow(digit as u32) * multiply, result)
                        });
                Ok(result.1)
            }
        }
    }

    fn calculate_digit(num: u32, base: u32) -> usize {
        let mut pow = 0;
        while num >= base.pow(pow + 1) {
            pow += 1;
        }
        pow += 1;
        return pow as usize;
    }

    fn calculate_multiply(rest: u32, to_base: u32, digit: u32) -> u32 {
        let mut multiply = 0;
        while rest >= to_base.pow(digit as u32) * (multiply + 1) {
            multiply += 1;
        }
        multiply
    }

    convert(
        &number
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>()[..],
        10,
        2,
    )
}
