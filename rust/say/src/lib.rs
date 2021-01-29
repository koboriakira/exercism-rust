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
        1000..=999999 => {
            let thousand = encode(n / 1000) + " thousand";
            if n % 1000 == 0 {
                thousand
            } else {
                thousand + " " + &encode(n % 1000)
            }
        }
        1_000_000..=999_999_999 => {
            let million = encode(n / 1_000_000) + " million";
            if n % 1_000_000 == 0 {
                million
            } else {
                million + " " + &encode(n % 1_000_000)
            }
        }
        1_000_000_000..=999_999_999_999 => {
            let billion = encode(n / 1_000_000_000) + " billion";
            if n % 1_000_000_000 == 0 {
                billion
            } else {
                billion + " " + &encode(n % 1_000_000_000)
            }
        }
        1_000_000_000_000..=999_999_999_999_999 => {
            let trillion = encode(n / 1_000_000_000_000) + " trillion";
            if n % 1_000_000_000_000 == 0 {
                trillion
            } else {
                trillion + " " + &encode(n % 1_000_000_000_000)
            }
        }
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            let quadrillion = encode(n / 1_000_000_000_000_000) + " quadrillion";
            if n % 1_000_000_000_000_000 == 0 {
                quadrillion
            } else {
                quadrillion + " " + &encode(n % 1_000_000_000_000_000)
            }
        }
        1_000_000_000_000_000_000..=u64::MAX => {
            let quintillion = encode(n / 1_000_000_000_000_000_000) + " quintillion";
            if n % 1_000_000_000_000_000_000 == 0 {
                quintillion
            } else {
                quintillion + " " + &encode(n % 1_000_000_000_000_000_000)
            }
        }
    }
}
