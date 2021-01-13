use std::fmt;

use fmt::{Display, Formatter, Result};

const BRACKETS: [[char; 2]; 3] = [['[', ']'], ['{', '}'], ['(', ')']];

struct Bracket {
    begin_char: char,
    end_char: char,
    begin_pos: Option<usize>,
    end_pos: Option<usize>,
    _secret: (),
}

impl Display for Bracket {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.begin_char, self.end_char)
    }
}

impl Bracket {
    fn has_nothing(&self) -> bool {
        self.begin_pos.is_none() && self.end_pos.is_none()
    }

    fn is_illegal(&self) -> bool {
        if self.begin_pos.is_none()
            || self.end_pos.is_none()
            || self.begin_pos.unwrap() > self.end_pos.unwrap()
        {
            true
        } else {
            let brackets = BRACKETS
                .iter()
                .find(|&predicate| predicate[0] == self.begin_char)
                .unwrap();
            println!("brackets: {:?}, end_bracket: {}", brackets, brackets[1]);
            self.end_char != brackets[1]
        }
    }

    fn remove_brackets<'a>(&self, string: &'a str) -> &'a str {
        &string[self.begin_pos.unwrap() + 1..self.end_pos.unwrap()]
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut string = string;
    loop {
        let bracket = find_bracket(string);
        println!("string: {}    bracket: {}", string, bracket);

        if bracket.has_nothing() {
            println!("has_nothing");
            return true;
        }
        if bracket.is_illegal() {
            println!("is_illegal");
            return false;
        }
        string = bracket.remove_brackets(&string);
    }
}

fn find_bracket(string: &str) -> Bracket {
    let begin_pos = string
        .chars()
        .position(|c| c == '[' || c == '{' || c == '(');
    let begin_char = if begin_pos.is_some() {
        string
            .chars()
            .skip(begin_pos.unwrap())
            .take(1)
            .collect::<Vec<char>>()[0]
    } else {
        ' '
    };
    let end_pos = string
        .chars()
        .rev()
        .position(|c| c == ']' || c == '}' || c == ')');
    let end_pos = if end_pos.is_none() {
        None
    } else {
        println!("len: {}, tmp_end_pos: {}", &string.len(), end_pos.unwrap());
        Some(&string.len() - end_pos.unwrap() - 1)
    };
    let end_char = if end_pos.is_some() {
        string
            .chars()
            .skip(end_pos.unwrap())
            .take(1)
            .collect::<Vec<char>>()[0]
    } else {
        ' '
    };
    Bracket {
        begin_char: begin_char,
        begin_pos: begin_pos,
        end_char: end_char,
        end_pos: end_pos,
        _secret: (),
    }
}
