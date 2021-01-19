/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    match sum(code) {
        Ok(sum) => sum % 10 == 0,
        Err(_) => false,
    }
}

fn sum(code: &str) -> Result<u32, String> {
    fn validate(code: &str) -> Result<&str, String> {
        if code.trim() == "0" {
            return Err(String::from("Single zero is error"));
        }
        match code.chars().find(|c| !c.eq(&' ') && !c.is_digit(10)) {
            Some(err_char) => Err(format!("{} is error", err_char)),
            None => Ok(code),
        }
    }

    fn double_if_index_is_even(index: usize, digit: u32) -> u32 {
        match (index % 2 != 0, digit >= 5) {
            (false, _) => digit,
            (true, false) => digit * 2,
            (true, true) => digit * 2 - 9,
        }
    }

    validate(code).and_then(|code| {
        Ok(code
            .chars()
            .filter(|c| c.is_digit(10))
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .map(|(i, d)| double_if_index_is_even(i, d))
            .sum::<u32>())
    })
}
