pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if Some('(') != stack.pop() {
                    return false;
                }
            }
            ']' => {
                if Some('[') != stack.pop() {
                    return false;
                }
            }
            '}' => {
                if Some('{') != stack.pop() {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
