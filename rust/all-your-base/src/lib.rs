#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
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

            let result = (0..digits)
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
    while num > base.pow(pow + 1) {
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
