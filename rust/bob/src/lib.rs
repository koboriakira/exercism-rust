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
        // _ => panic!("Don't have Message type"),
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
    message.ends_with("?")
}

fn is_yell(message: &str) -> bool {
    message.chars().any(char::is_alphabetic) && message == message.to_uppercase()
}

fn is_nothing(message: &str) -> bool {
    message.is_empty()
}
