const BRACKETS: [[char; 2]; 3] = [['[', ']'], ['{', '}'], ['(', ')']];

pub fn brackets_are_balanced(string: &str) -> bool {
    println!("string: {}", string);
    let mut string = string.trim();
    let begin_pos = string
        .chars()
        .position(|c| c == '[' || c == '{' || c == '(');
    if begin_pos.is_some() {
        let begin_pos = begin_pos.unwrap();
        let begin_char = string
            .chars()
            .skip(begin_pos)
            .take(1)
            .collect::<Vec<char>>()[0];
        println!("begin_char: {}  begin_pos: {}", begin_char, begin_pos);
        let end_char = BRACKETS.iter().find(|b| b[0] == begin_char).unwrap()[1];
        if has_end_char(string, begin_pos) {
            println!("has end char");
            return false;
        }
        let end_pos = get_end_pos(string, begin_char, begin_pos, end_char);
        if end_pos.is_none() {
            return false;
        }
        let end_pos = end_pos.unwrap();
        println!("end_char: {}  end_pos: {}", end_char, end_pos);
        let inline_string = &string[begin_pos + 1..end_pos];
        println!("inline_string: {}", inline_string);
        if !brackets_are_balanced(inline_string) {
            return false;
        }
        string = &string[end_pos + 1..string.len()];
        return brackets_are_balanced(string);
    } else {
        println!("begin_pos is none");
        let end_pos = string
            .chars()
            .rev()
            .position(|c| c == ']' || c == '}' || c == ')');
        println!("end_pos is none: {}", end_pos.is_none());
        return end_pos.is_none();
    }
    // let mut string = string;
    // loop {
    //     let bracket = find_bracket(string);
    //     println!("string: {}    bracket: {}", string, bracket);

    //     if bracket.has_nothing() {
    //         println!("has_nothing");
    //         return true;
    //     }
    //     if bracket.is_illegal() {
    //         println!("is_illegal");
    //         return false;
    //     }
    //     string = bracket.remove_brackets(&string);
    // }
}

fn has_end_char(string: &str, end: usize) -> bool {
    string[0..end]
        .chars()
        .find(|&c| c == ']' || c == '}' || c == ')')
        .is_some()
}

fn get_end_pos(string: &str, begin_char: char, begin_pos: usize, end_char: char) -> Option<usize> {
    // println!(
    //     "string, begin_char, begin_pos, end_char: {} {} {} {}",
    //     string, begin_char, begin_pos, end_char
    // );
    let mut begin_char_count = 1;
    for i in begin_pos + 1..string.len() {
        let char = string.chars().nth(i).unwrap();
        // println!("char: {}", char);
        if char == begin_char {
            begin_char_count += 1;
        } else if char == end_char {
            begin_char_count -= 1;
        }
        if begin_char_count == 0 {
            return Some(i);
        }
    }
    return None;
    // let end_pos = string[begin_pos..].chars().position(|c| c == end_char);
}
