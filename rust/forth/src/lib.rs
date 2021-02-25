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

#[derive(Clone, Debug, PartialEq)]
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
    BuiltIn(Vec<Element>),
}

impl Element {
    fn needs_one_argument(&self) -> bool {
        [Element::Duplicate, Element::Drop].contains(self)
    }

    fn needs_two_arguments(&self) -> bool {
        [
            Element::Add,
            Element::Subtract,
            Element::Multiply,
            Element::Divide,
            Element::Swap,
            Element::Over,
        ]
        .contains(self)
    }
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

    pub fn pop(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or_else(|| Error::StackUnderflow)
    }

    pub fn push_multiple(&mut self, values: &[i32]) -> Result<(), Error> {
        for value in values {
            self.stack.push(*value);
        }
        Ok(())
    }

    pub fn push(&mut self, value: i32) -> Result<(), Error> {
        self.stack.push(value);
        Ok(())
    }

    pub fn close(&self) -> Result<(), Error> {
        match self.mode_build_builtin {
            true => Err(Error::InvalidWord),
            _ => Ok(()),
        }
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        input
            .to_lowercase()
            .split(" ")
            .into_iter()
            .try_for_each(|el| {
                println!("word: {}", el);
                match (self.mode_build_builtin, el) {
                    (true, ":") | (false, ";") => Err(Error::InvalidWord),
                    // start to create builtin
                    (false, ":") => {
                        self.mode_build_builtin = true;
                        Ok(())
                    }
                    // finish to create builtin and generate it
                    (true, ";") => self.create_builtin(),
                    // stack for builtin
                    (true, el) => {
                        self.builtin_stack.push(el.to_string());
                        Ok(())
                    }
                    // execute
                    (false, el) => self.evaluate(el).and_then(|element| self.execute(element)),
                }
            })
            .and_then(|_| self.close())
    }

    fn create_builtin(&mut self) -> Result<(), Error> {
        self.get_command_name().and_then(|command_name| {
            self.builtin_stack
                .clone()
                .into_iter()
                .enumerate()
                .try_for_each(|(idx, el)| {
                    self.evaluate(&el).and_then(|element| {
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
                    })
                })
                .and_then(|_| {
                    self.mode_build_builtin = false;
                    self.builtin_stack.clear();
                    println!("Build built-in function: {:?}", self.builtins);
                    Ok(())
                })
        })
    }

    fn get_command_name(&mut self) -> Result<String, Error> {
        let command_name = self.builtin_stack.remove(0);
        if convert_to_integer(&command_name).is_ok() {
            return Err(Error::InvalidWord);
        }
        return Ok(command_name);
    }

    fn execute(&mut self, element: Element) -> Result<(), Error> {
        println!("Execute: {:?} => {:?}", self.stack, element);
        match element {
            Element::Number(num) => {
                self.stack.push(num);
                Ok(())
            }
            Element::BuiltIn(elements) => elements
                .into_iter()
                .try_for_each(|element| self.execute(element.clone())),
            element if element.needs_one_argument() => self.pop().and_then(|val| match element {
                Element::Duplicate => self.push_multiple(&[val; 2]),
                Element::Drop => Ok(()),
                _ => unreachable!(),
            }),
            element if element.needs_two_arguments() => self.pop().and_then(|b| {
                self.pop().and_then(|a| match element {
                    Element::Add => self.push(a + b),
                    Element::Subtract => self.push(a - b),
                    Element::Multiply => self.push(a * b),
                    Element::Divide => match b == 0 {
                        true => Err(Error::DivisionByZero),
                        false => self.push(a / b),
                    },
                    Element::Swap => self.push_multiple(&[b, a]),
                    Element::Over => self.push_multiple(&[a, b, a]),
                    _ => unreachable!(),
                })
            }),
            _ => unreachable!(),
        }
    }

    fn evaluate(&self, el: &str) -> Result<Element, Error> {
        match el {
            el if self.builtins.get(el).is_some() => {
                Ok(Element::BuiltIn(self.builtins.get(el).unwrap().clone()))
            }
            "+" => Ok(Element::Add),
            "-" => Ok(Element::Subtract),
            "*" => Ok(Element::Multiply),
            "/" => Ok(Element::Divide),
            "dup" => Ok(Element::Duplicate),
            "drop" => Ok(Element::Drop),
            "swap" => Ok(Element::Swap),
            "over" => Ok(Element::Over),
            el => match convert_to_integer(el) {
                Ok(num) => Ok(Element::Number(num)),
                _ => Err(Error::UnknownWord),
            },
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
