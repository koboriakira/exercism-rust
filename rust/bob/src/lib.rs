use core::panic;
use regex::Regex;

enum Message {
    AnythingElse,
    Question,
    Yell,
    YellQuestion,
    Nothing,
}

pub fn reply(message: &str) -> &str {
    match analyse(message) {
        Message::Question => "Sure.",
        Message::AnythingElse => "Whatever.",
        Message::Yell => "Whoa, chill out!",
        Message::YellQuestion => "Calm down, I know what I'm doing!",
        Message::Nothing => "Fine. Be that way!",
        _ => panic!("Don't have Message type"),
    }
}

fn analyse(message: &str) -> Message {
    let message = message.trim();

    if is_nothing(message) {
        return Message::Nothing;
    }
    println!("message: {}", message);
    let is_question = is_question(message);
    let is_yell = is_yell(message);
    println!("is_question: {}", is_question);
    println!("is_yell: {}", is_yell);

    if is_question {
        if is_yell {
            Message::YellQuestion
        } else {
            Message::Question
        }
    } else {
        if is_yell {
            Message::Yell
        } else {
            Message::AnythingElse
        }
    }
}

fn is_question(message: &str) -> bool {
    message.chars().rev().next().unwrap().eq(&'?')
}

fn is_yell(message: &str) -> bool {
    Regex::new(r"[A-Z]").unwrap().is_match(message)
        && Regex::new(r"^[0-9A-Z,.\s\-!?%'^*@#$(*^]*$")
            .unwrap()
            .is_match(message)
}

fn is_nothing(message: &str) -> bool {
    Regex::new(r"^[0-9\s]*$").unwrap().is_match(message)
}
