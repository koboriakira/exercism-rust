use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    mode_build_builtin: bool,
    builtin_stack: Vec<String>,
    builtins: HashMap<String, Vec<Element>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Clone, Debug)]
enum Element {
    Number(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Duplicate,
    Drop,
    Swap,
    Over,
    StartBuildingBuiltIn,
    EndBuildingBuiltIn,
    BuiltIn(Vec<Element>),
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            mode_build_builtin: false,
            builtin_stack: vec![],
            builtins: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let result = input
            .to_lowercase()
            .split(" ")
            .into_iter()
            .try_for_each(|el| {
                println!("word: {}", el);
                if self.mode_build_builtin {
                    match ";".eq(el) {
                        false => {
                            self.builtin_stack.push(el.to_string());
                            Ok(())
                        }
                        true => {
                            let command_name = self.builtin_stack.remove(0);
                            if convert_to_integer(&command_name).is_ok() {
                                return Err(Error::InvalidWord);
                            }
                            let result = self
                                .builtin_stack
                                .clone()
                                .into_iter()
                                .enumerate()
                                .try_for_each(|(idx, el)| match self.evaluate(&el) {
                                    Ok(element) => {
                                        println!("Buildin Element: {:?}", element);
                                        if idx == 0 {
                                            self.builtins
                                                .entry(command_name.clone())
                                                .or_insert_with(|| vec![])
                                                .clear();
                                        }
                                        self.builtins
                                            .entry(command_name.clone())
                                            .or_insert_with(|| vec![])
                                            .push(element);
                                        Ok(())
                                    }
                                    Err(err) => Err(err),
                                });
                            self.mode_build_builtin = false;
                            self.builtin_stack.clear();
                            println!("Build built-in function: {:?}", self.builtins);
                            result
                        }
                    }
                } else {
                    match self.evaluate(el) {
                        Ok(element) => self.execute(element),
                        Err(err) => Err(err),
                    }
                }
            });
        if self.mode_build_builtin {
            return Err(Error::InvalidWord);
        }
        result
    }

    fn execute(&mut self, element: Element) -> Result<(), Error> {
        println!("Execute: {:?} => {:?}", self.stack, element);
        match element {
            Element::StartBuildingBuiltIn => match self.mode_build_builtin {
                true => Err(Error::InvalidWord),
                false => {
                    self.mode_build_builtin = true;
                    Ok(())
                }
            },
            Element::EndBuildingBuiltIn => match self.mode_build_builtin {
                false => Err(Error::InvalidWord),
                true => {
                    self.mode_build_builtin = false;
                    Ok(())
                }
            },
            Element::Number(num) => {
                self.stack.push(num);
                Ok(())
            }
            Element::Add => match (self.stack.pop(), self.stack.pop()) {
                (Some(b), Some(a)) => {
                    self.stack.push(a + b);
                    Ok(())
                }
                _ => Err(Error::StackUnderflow),
            },
            Element::Subtract => match (self.stack.pop(), self.stack.pop()) {
                (Some(b), Some(a)) => {
                    self.stack.push(a - b);
                    Ok(())
                }
                _ => Err(Error::StackUnderflow),
            },
            Element::Multiply => match (self.stack.pop(), self.stack.pop()) {
                (Some(b), Some(a)) => {
                    self.stack.push(a * b);
                    Ok(())
                }
                _ => Err(Error::StackUnderflow),
            },
            Element::Divide => match (self.stack.pop(), self.stack.pop()) {
                (Some(0), Some(_)) => Err(Error::DivisionByZero),
                (Some(b), Some(a)) => {
                    self.stack.push(a / b);
                    Ok(())
                }
                _ => Err(Error::StackUnderflow),
            },
            Element::Duplicate => match &self.stack.clone().last() {
                Some(val) => {
                    self.stack.push(*val.clone());
                    Ok(())
                }
                _ => Err(Error::StackUnderflow),
            },
            Element::Drop => match self.stack.pop() {
                Some(_) => Ok(()),
                _ => Err(Error::StackUnderflow),
            },
            Element::Swap => match (self.stack.pop(), self.stack.pop()) {
                (Some(b), Some(a)) => {
                    self.stack.push(b);
                    self.stack.push(a);
                    Ok(())
                }
                _ => Err(Error::StackUnderflow),
            },
            Element::Over => match (self.stack.pop(), self.stack.pop()) {
                (Some(b), Some(a)) => {
                    self.stack.push(a);
                    self.stack.push(b);
                    self.stack.push(a);
                    Ok(())
                }
                _ => Err(Error::StackUnderflow),
            },
            Element::BuiltIn(elements) => elements
                .into_iter()
                .try_for_each(|element| self.execute(element.clone())),
            _ => Ok(()),
        }
    }

    fn evaluate(&self, el: &str) -> Result<Element, Error> {
        if let Some(elements) = self.builtins.get(&el.to_lowercase()) {
            return Ok(Element::BuiltIn(elements.clone()));
        }

        match &el.to_lowercase()[..] {
            "+" => return Ok(Element::Add),
            "-" => return Ok(Element::Subtract),
            "*" => return Ok(Element::Multiply),
            "/" => return Ok(Element::Divide),
            "dup" => return Ok(Element::Duplicate),
            "drop" => return Ok(Element::Drop),
            "swap" => return Ok(Element::Swap),
            "over" => return Ok(Element::Over),
            ":" => return Ok(Element::StartBuildingBuiltIn),
            ";" => return Ok(Element::EndBuildingBuiltIn),
            _ => {}
        }

        match convert_to_integer(el) {
            Ok(num) => Ok(Element::Number(num)),
            _ => Err(Error::UnknownWord),
        }
    }
}

fn convert_to_integer(el: &str) -> Result<i32, ()> {
    el.chars()
        .rev()
        .enumerate()
        .try_fold(0_i32, |acc, (idx, ch)| match ch.to_digit(10) {
            Some(val) => Ok(val as i32 * 10_i32.pow(idx as u32) + acc),
            _ => Err(()),
        })
}
