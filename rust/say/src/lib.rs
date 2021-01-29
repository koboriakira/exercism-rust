pub fn encode(n: u64) -> String {
    match n {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        60 => String::from("sixty"),
        70 => String::from("seventy"),
        80 => String::from("eighty"),
        90 => String::from("ninety"),
        21..=99 => encode((n / 10) * 10) + "-" + &encode(n % 10),
        100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900 => encode(n / 100) + " hundred",
        101..=999 => encode(n / 100) + " hundred " + &encode(n % 100),
        1_000..=999_999 => large_digit(n, 1000, "thousand"),
        1_000_000..=999_999_999 => large_digit(n, 1_000_000, "million"),
        1_000_000_000..=999_999_999_999 => large_digit(n, 1_000_000_000, "billion"),
        1_000_000_000_000..=999_999_999_999_999 => large_digit(n, 1_000_000_000_000, "trillion"),
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            large_digit(n, 1_000_000_000_000_000, "quadrillion")
        }
        1_000_000_000_000_000_000..=u64::MAX => {
            large_digit(n, 1_000_000_000_000_000_000, "quintillion")
        }
    }
}

fn large_digit(n: u64, digit: u64, digit_name: &str) -> String {
    let result = encode(n / digit) + " " + digit_name;
    if n % digit == 0 {
        result
    } else {
        result + " " + &encode(n % digit)
    }
}
