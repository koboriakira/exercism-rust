const BRACKETS: [[char; 2]; 3] = [['[', ']'], ['{', '}'], ['(', ')']];

pub fn brackets_are_balanced(string: &str) -> bool {
    let string = string.trim();
    let begin_pos = string
        .chars()
        .position(|c| c == '[' || c == '{' || c == '(');
    if begin_pos.is_none() {
        return !has_end_char(string, string.len());
    }

    let begin_pos = begin_pos.unwrap();
    if has_end_char(string, begin_pos) {
        return false;
    }

    let begin_char = get_char(string, begin_pos);
    let end_pos = get_end_pos(string, begin_char, begin_pos);
    if end_pos.is_none() {
        return false;
    }

    let end_pos = end_pos.unwrap();
    let inline_string = &string[begin_pos + 1..end_pos];
    if !brackets_are_balanced(inline_string) {
        return false;
    }

    let rest_string = &string[end_pos + 1..string.len()];
    return brackets_are_balanced(rest_string);
}

fn get_char(string: &str, pos: usize) -> char {
    string.chars().skip(pos).take(1).collect::<Vec<char>>()[0]
}

fn get_end_char(begin_char: char) -> char {
    BRACKETS.iter().find(|b| b[0] == begin_char).unwrap()[1]
}

fn has_end_char(string: &str, end: usize) -> bool {
    string[0..end]
        .chars()
        .find(|&c| c == ']' || c == '}' || c == ')')
        .is_some()
}

fn get_end_pos(string: &str, begin_char: char, begin_pos: usize) -> Option<usize> {
    let end_char = get_end_char(begin_char);
    let mut begin_char_count = 1;
    for i in begin_pos + 1..string.len() {
        let possible_bracket = string.chars().nth(i).unwrap();
        begin_char_count += count_begin_or_end(possible_bracket, begin_char, end_char);
        if begin_char_count == 0 {
            return Some(i);
        }
    }
    None
}

fn count_begin_or_end(possible_bracket: char, begin_char: char, end_char: char) -> i32 {
    if possible_bracket == begin_char {
        1
    } else if possible_bracket == end_char {
        -1
    } else {
        0
    }
}
