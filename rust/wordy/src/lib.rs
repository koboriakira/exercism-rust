pub struct WordProblem {
    value: i32,
    last_word_type: WordType,
    is_calculated: bool,
}

trait Try {
    fn new() -> Self;
}

impl Try for WordProblem {
    fn new() -> Self {
        Self {
            value: 0,
            last_word_type: WordType::None,
            is_calculated: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WordType {
    None,
    Number(i32),
    Plus,
    Minus,
    Multiple,
    Divided,
}

impl WordType {
    fn is_operator(&self) -> bool {
        [
            WordType::Plus,
            WordType::Minus,
            WordType::Multiple,
            WordType::Divided,
        ]
        .contains(&self)
    }

    fn is_number(&self) -> bool {
        match self {
            WordType::Number(_) => true,
            _ => false,
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let result = command
        .split(" ")
        .try_fold(WordProblem::new(), |acc, word| {
            word_type(word, acc.last_word_type).and_then(|word_type| match word_type {
                WordType::Number(cur) => calculate(cur, acc).and_then(|res| {
                    Ok(WordProblem {
                        value: res,
                        last_word_type: word_type,
                        is_calculated: true,
                    })
                }),
                word_type => Ok(WordProblem {
                    last_word_type: word_type,
                    ..acc
                }),
            })
        })
        .and_then(|res| match (res.is_calculated, res.last_word_type) {
            (true, WordType::Number(_)) => return Ok(res.value),
            _ => return Err(()),
        });
    match result {
        Ok(res) => Some(res),
        _ => None,
    }
}

fn calculate(cur: i32, acc: WordProblem) -> Result<i32, ()> {
    println!(
        "calculate: {} {:?} {:?}",
        acc.value, cur, acc.last_word_type
    );
    match acc.last_word_type {
        WordType::None => Ok(cur),
        WordType::Plus => Ok(acc.value + cur),
        WordType::Minus => Ok(acc.value - cur),
        WordType::Multiple => Ok(acc.value * cur),
        WordType::Divided => Ok(acc.value / cur),
        _ => Err(()),
    }
}

fn word_type(word: &str, last_word_type: WordType) -> Result<WordType, ()> {
    let word_type = match word.trim_end_matches("?") {
        "plus" => WordType::Plus,
        "minus" => WordType::Minus,
        "multiplied" => WordType::Multiple,
        "divided" => WordType::Divided,
        "by" => last_word_type,
        word => {
            if let Some(val) = convert_integer(word) {
                WordType::Number(val)
            } else {
                WordType::None
            }
        }
    };

    println!(
        "word_type:{:?} last_word_type:{:?}",
        word_type, last_word_type
    );

    if word_type.is_operator() && last_word_type.is_operator() && word != "by" {
        return Err(());
    } else if word_type.is_number() && last_word_type.is_number() {
        return Err(());
    }

    Ok(word_type)
}

fn convert_integer(word: &str) -> Option<i32> {
    match i32::from_str_radix(word, 10) {
        Ok(res) => Some(res),
        _ => None,
    }
}
