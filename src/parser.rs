//pub mod scanner;
use crate::scanner;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
#[allow(dead_code)]
enum Type {
    Int,
    Float,
    Str,
    Var,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Terminator,
    Exit,
    Error,
    SpecificError(String),
    NotImplementedError,
    NameError,
}

#[derive(Debug)]
struct Data {
    type_meta: Type,
    value_meta: Value,
}

struct State {
    vars: HashMap<String, Data>,
}

pub struct Parser {
    input: scanner::Input,
    state: State,
}

impl Parser {
    pub fn build_new() -> Parser {
        let parser = Parser {
            input: scanner::Input {
                stream: String::from(""),
                current: scanner::Token::NewLine,
                history: VecDeque::new(),
            },
            state: State {
                vars: HashMap::new(),
            },
        };
        return parser;
    }

    /* program ::= exit | state | var_ref_print | statement | program statement
     *
     */
    pub fn parse_program(&mut self) {
        loop {
            // update current token
            self.input.get_next_token(true);

            match self.input.current {
                scanner::Token::Exit => break,
                scanner::Token::State => {
                    self.parse_state();
                },
                scanner::Token::NewLine => {
                    self.parse_newline();
                }
                _ => {
                    let value: Value = self.parse_statement();
                    match value {
                        Value::Exit => break,
                        _ => {
                            // TODO: catch early termination -> syntax error?
                            println!("Result: {:?}", value)
                        }
                    }
                }
            };
        }
    }

    #[allow(dead_code)]
    fn parse_newline(&mut self) -> Value {
        // consume token
        self.input.get_next_token(true);
        return Value::Terminator;
    }

    /*
     * TODO: could serve purpose for logging in the future
     */
    #[allow(dead_code)]
    fn parse_exit(&mut self) -> Value {
        self.input.get_next_token(true);
        return Value::Exit;
    }

    /* debugging function
     * display current state
     */
    fn parse_state(&mut self) -> Value {
        // consume token
        self.input.get_next_token(true);
        println!("current state:");
        for (key, value) in &self.state.vars {
            println!("{}: {:?}", key, value);
        }
        println!("current stream: {:?}", self.input.stream);
        return Value::Terminator;
    }

    /* statement ::= expr | var = expr | epsilon
     * TODO think about utility of epsilon
     * TODO think about how to terminate successfully
     * TODO thinking about incomplete lines - currently, trying to allow for
     * just a single var, but need to enable different incomplete statements to
     * prompt on a newline for completion
     */
    fn parse_statement(&mut self) -> Value {
        match self.input.current {
            scanner::Token::Variable(_) => {
                match self.input.look_ahead(true) {
                    scanner::Token::Equals => return self.parse_assign(),
                    _ => return self.parse_var_ref(),
                }
            },
            _ => return self.parse_expression(),
        }
    }

    fn parse_assign(&mut self) -> Value {
        let var_name: String;
        if let scanner::Token::Variable(var) = &mut self.input.current {
            var_name = var.to_string();
        } else {
            return Value::Error;
        }
        self.input.get_next_token(true);
        self.input.get_next_token(true);
        let var_value = self.parse_expression();
        let var_type: Type;
        match var_value {
            Value::Int(_) => var_type = Type::Int,
            Value::Float(_) => var_type = Type::Float,
            _ => return Value::Error,
        };
        let data = Data {
            value_meta: var_value,
            type_meta: var_type,
        };
        self.state.vars.insert(var_name, data);
        return Value::Terminator;
    }

    /* expr ::= term | expr + term | expr - term
     *
     */
    fn parse_expression(&mut self) -> Value {
        let mut return_value = self.parse_term();
        loop {
            match &self.input.current {
                /*
                scanner::Token::WhiteSpace(_) => {
                    // println!("parse_expr consume whitespace");
                    self.input.get_next_token(true); // consume whitespace
                    // println!("current is now {:?}", &self.input.current);
                }
                */
                scanner::Token::Plus => {
                    // println!("handling plus");
                    self.input.get_next_token(true);
                    if let Value::Int(val_int) = return_value {
                        if let Value::Int(val_parsed) = self.parse_term() {
                            return_value = Value::Int(val_int + val_parsed);
                        }
                    }
                }
                scanner::Token::Minus => {
                    self.input.get_next_token(true);
                    if let Value::Int(val_int) = return_value {
                        if let Value::Int(val_parsed) = self.parse_term() {
                            return_value = Value::Int(val_int + val_parsed);
                        }
                    }
                }
                _ => break,
            }
        }
        return return_value;
    }

    /* term ::= power | term * power | term / power
     *
     */
    fn parse_term(&mut self) -> Value {
        let mut return_value = self.parse_power();
        loop {
            match self.input.current {
                scanner::Token::Multiply => {
                    self.input.get_next_token(true);
                    if let Value::Int(val_int) = return_value {
                        if let Value::Int(val_parsed) = self.parse_power() {
                            return_value = Value::Int(val_int * val_parsed);
                        }
                    }
                }
                scanner::Token::Divide => {
                    self.input.get_next_token(true);
                    if let Value::Int(val_int) = return_value {
                        if let Value::Int(val_parsed) = self.parse_power() {
                            return_value = Value::Int(val_int / val_parsed);
                        }
                    }
                }
                _ => break,
            }
        }
        return return_value;
    }

    /* power ::= factor | factor ** power
     * TODO handle floats
     */
    fn parse_power(&mut self) -> Value {
        let factor: Value = self.parse_factor();
        match self.input.current {
            scanner::Token::NewLine => {
                return factor;
            }
            scanner::Token::Exponent => {
                self.input.get_next_token(true); // consume operator
                let power: Value = self.parse_power();
                if let Value::Int(base_int) = factor {
                    if let Value::Int(power_int) = power {
                        let total: u32 = power_int as u32; // TODO should make this panic
                        match base_int.checked_pow(total) {
                            Some(pow) => return Value::Int(pow),
                            _ => return Value::Error
                        };
                    }
                }
                return Value::Error;
            }
            _ => return factor,
        };
    }

    /* factor ::= var_ref | number | (exp) | -exp
     * TODO how to parse negative number?
     */
    fn parse_factor(&mut self) -> Value {
        match self.input.current {
            scanner::Token::OpenParen => return self.parse_parens(),
            scanner::Token::Int(_) | scanner::Token::Float(_) => return self.parse_number(),
            scanner::Token::Variable(_) => return self.parse_var_ref(),
            _ => return Value::Error,
        };
    }

    fn parse_var_ref(&mut self) -> Value {
        let var = self.input.current.clone();
        match var {
            scanner::Token::Variable(name) => {
                self.input.get_next_token(true);
                match self.state.vars.get(&name) {
                    Some(data) => {
                        return data.value_meta.clone();
                    },
                    None => return Value::NameError,
                }
            },
            _ => {
                    self.input.get_next_token(true);
                    return Value::SpecificError(String::from("Unknown error: parsing var ref"));
            },
        };
    }

    /* number ::= int | float
     *
     */
    fn parse_number(&mut self) -> Value {
        match self.input.current {
            scanner::Token::Int(_) => return self.parse_int(),
            scanner::Token::Float(_) => return self.parse_float(),
            _ => {
                self.input.get_next_token(true);
                return Value::Error;
            }
        };
    }

    fn parse_int(&mut self) -> Value {
        if let scanner::Token::Int(val) = self.input.current {
            self.input.get_next_token(true);
            return Value::Int(val);
        } else {
            self.input.get_next_token(true);
            return Value::Error;
        }
    }

    fn parse_float(&mut self) -> Value {
        if let scanner::Token::Float(val) = self.input.current {
            self.input.get_next_token(true);
            return Value::Float(val);
        } else {
            self.input.get_next_token(true);
            return Value::Error;
        }
    }

    fn parse_parens(&mut self) -> Value {
        self.input.get_next_token(true); // consume "("
        let value: Value = self.parse_expression();
        self.input.get_next_token(true); // consume ")"
        return value;
    }

    /* for debugging/etc - simply repeats tokens back to user, 1 per line
    */
    #[allow(dead_code)]
    fn repeat_tokens(&mut self) {
        loop {
            let token = self.input.get_next_token(true);
            println!("{:?}", token);
        }
    }
}

