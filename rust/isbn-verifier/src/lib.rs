/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    get_nums(isbn).and_then(|res| validate_sum(&res)).is_ok()
}

fn get_nums(isbn: &str) -> Result<Vec<u32>, ()> {
    let nums = isbn
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| match (c, idx) {
            ('X', idx) if idx == isbn.len() - 1 => Some(10),
            (c, _) => c.to_digit(10),
        })
        .collect::<Vec<u32>>();

    match nums.len() {
        10 => Ok(nums),
        _ => Err(()),
    }
}

fn validate_sum(nums: &Vec<u32>) -> Result<(), ()> {
    let sum = nums
        .iter()
        .enumerate()
        .fold(0_u32, |acc, (idx, num)| acc + num * (10 - idx as u32));

    match sum % 11 {
        0 => Ok(()),
        _ => Err(()),
    }
}
