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
                    // finish to create builtin
                    (true, ";") => self.get_command_name().and_then(|command_name| {
                        self.builtin_stack
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
                            })
                            .and_then(|_| {
                                self.mode_build_builtin = false;
                                self.builtin_stack.clear();
                                println!("Build built-in function: {:?}", self.builtins);
                                Ok(())
                            })
                    }),
                    // stack for builtin
                    (true, el) => {
                        self.builtin_stack.push(el.to_string());
                        Ok(())
                    }
                    // execute
                    (false, el) => self.evaluate(el).and_then(|element| self.execute(element)),
                }
            })
            .and_then(|_| match self.mode_build_builtin {
                true => Err(Error::InvalidWord),
                _ => Ok(()),
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
            element if element.needs_one_argument() => {
                match (element, self.stack.pop()) {
                    (Element::Duplicate, Some(val)) => {
                        self.stack.push(val);
                        self.stack.push(val);
                    }
                    (Element::Drop, Some(_)) => {}
                    _ => return Err(Error::StackUnderflow),
                };
                Ok(())
            }
            element if element.needs_two_arguments() => {
                match (element, self.stack.pop(), self.stack.pop()) {
                    (Element::Add, Some(b), Some(a)) => self.stack.push(a + b),
                    (Element::Subtract, Some(b), Some(a)) => self.stack.push(a - b),
                    (Element::Multiply, Some(b), Some(a)) => self.stack.push(a * b),
                    (Element::Divide, Some(0), Some(_)) => return Err(Error::DivisionByZero),
                    (Element::Divide, Some(b), Some(a)) => self.stack.push(a / b),
                    (Element::Swap, Some(b), Some(a)) => {
                        self.stack.push(b);
                        self.stack.push(a);
                    }
                    (Element::Over, Some(b), Some(a)) => {
                        self.stack.push(a);
                        self.stack.push(b);
                        self.stack.push(a);
                    }
                    _ => return Err(Error::StackUnderflow),
                }
                Ok(())
            }
            _ => panic!(""),
        }
    }

    fn evaluate(&self, el: &str) -> Result<Element, Error> {
        if let Some(elements) = self.builtins.get(el) {
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
