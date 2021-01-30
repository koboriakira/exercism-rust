use core::panic;

pub fn encode(source: &str) -> String {
    if source.eq(&String::from("")) {
        return String::from("");
    }

    let result = source
        .chars()
        .into_iter()
        .fold((String::new(), None, 1_u32), |acc, c| {
            let (string, last_char, char_count): (String, Option<char>, u32) = acc;
            if let Some(last_char) = last_char {
                match c == last_char {
                    true => (string, Some(last_char), char_count + 1),
                    false => (
                        string + &count_str(char_count) + &last_char.to_string(),
                        Some(c),
                        1,
                    ),
                }
            } else {
                (string, Some(c), 1)
            }
        });
    let result_string = result.0 + &count_str(result.2) + &result.1.unwrap().to_string();
    String::from(result_string)
}

pub fn decode(source: &str) -> String {
    let chars = source.chars();
    let bar = chars.map(add_comma_unless_numeric).collect::<String>();
    bar.split(",")
        .collect::<Vec<&str>>()
        .into_iter()
        .filter_map(|string| {
            if string.is_empty() {
                return None;
            }
            let (num, alphabet): (u32, char) = get_num_and_alphabet(string);
            let result = (0..num).fold(String::new(), |acc, _| acc + &String::from(alphabet));
            Some(result)
        })
        .collect::<Vec<String>>()
        .join("")
}

fn count_str(char_count: u32) -> String {
    match char_count {
        1 => String::from(""),
        _ => char_count.to_string(),
    }
}

fn add_comma_unless_numeric(c: char) -> String {
    if c.is_numeric() {
        return c.to_string();
    }
    [String::from(c), String::from(",")].join("").to_string()
}

fn get_num_and_alphabet(string: &str) -> (u32, char) {
    if string.len() == 1 {
        return (1, string.chars().nth(0).unwrap());
    }
    let mut count_str = String::from("");
    for (i, c) in string.chars().enumerate() {
        println!("{}, {}, {}", i, c, string.len());
        if i == string.len() - 1 {
            println!("{:?}", (count_str.parse::<u32>().unwrap(), c));
            return (count_str.parse::<u32>().unwrap(), c);
        }
        count_str = count_str + &c.to_string();
    }
    panic!()
}
