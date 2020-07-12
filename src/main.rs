pub mod scanner;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
#[allow(dead_code)]
enum Type {
    Int,
    Float,
    Str,
    Var,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Terminator,
    Exit,
    Error,
    NotImplementedError,
}

#[derive(Debug)]
struct Data {
    type_meta: Type,
    value_meta: Value,
}

struct State {
    vars: HashMap<String, Data>,
}

struct Parser {
    input: scanner::Input,
    state: State,
}

impl Parser {
    #[allow(dead_code)]
    fn parse_newline(&mut self) -> Value {
        // consume token
        self.input.get_next_token();
        return Value::Terminator;
    }

    /* program ::= exit | list | statement | program statement
     *
     */
    fn parse_program(&mut self) {
        println!("{:?}", &self.input.stream);
        loop {
            // update current token
            self.input.get_next_token();
            println!("{:?}", self.input.current); // for debugging

            // parse
            match self.input.current {
                scanner::Token::Exit => break,
                scanner::Token::List => {
                    self.parse_list();
                },
                _ => {
                    let value: Value = self.parse_statement();
                    match value {
                        Value::Exit => break,
                        _ => println!("{:?}", value)
                    }
                }

            };
        }
    }

    fn parse_exit(&mut self) -> Value {
        self.input.get_next_token();
        return Value::Exit;
    }

    fn parse_list(&mut self) -> Value {
        return Value::NotImplementedError;
    }

    /* statement ::= expr | id | id = expr | epsilon
     * TODO think about utility of epsilon
     * TODO think about how to terminate successfully
     * TODO thinking about incomplete lines - currently, trying to allow for
     * just a single var, but need to enable different incomplete statements to
     * prompt on a newline for completion
     */
    fn parse_statement(&mut self) -> Value {
        match self.input.current {
            scanner::Token::Exit => return self.parse_exit(),
            scanner::Token::List => return self.parse_list(),
            _ => match self.input.look_ahead() {
                scanner::Token::Equals => return self.parse_assign(),
                _ => return self.parse_expression(),
            },
        };
    }

    fn parse_assign(&mut self) -> Value {
        let var_name: String;
        if let scanner::Token::Variable(var) = &mut self.input.current {
            var_name = var.to_string();
        } else {
            return Value::Error;
        }
        self.input.get_next_token();
        self.input.get_next_token();
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
            match self.input.current {
                scanner::Token::Plus => {
                    self.input.get_next_token();
                    if let Value::Int(val_int) = return_value {
                        if let Value::Int(val_parsed) = self.parse_term() {
                            return_value = Value::Int(val_int + val_parsed);
                        }
                    }
                }
                scanner::Token::Minus => {
                    self.input.get_next_token();
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
                    self.input.get_next_token();
                    if let Value::Int(val_int) = return_value {
                        if let Value::Int(val_parsed) = self.parse_power() {
                            return_value = Value::Int(val_int * val_parsed);
                        }
                    }
                }
                scanner::Token::Divide => {
                    self.input.get_next_token();
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
     *
     */
    fn parse_power(&mut self) -> Value {
        let factor: Value = self.parse_factor();
        match self.input.current {
            scanner::Token::Exponent => {
                let power: Value = self.parse_power();

                // TODO: implement exponent function
                return Value::NotImplementedError;
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
        return Value::NotImplementedError;
    }

    /* number ::= int | float
     *
     */
    fn parse_number(&mut self) -> Value {
        match self.input.current {
            scanner::Token::Int(_) => return self.parse_int(),
            scanner::Token::Float(_) => return self.parse_float(),
            _ => {
                self.input.get_next_token();
                return Value::Error;
            }
        };
    }

    fn parse_int(&mut self) -> Value {
        if let scanner::Token::Int(val) = self.input.current {
            self.input.get_next_token();
            return Value::Int(val);
        } else {
            self.input.get_next_token();
            return Value::Error;
        }
    }

    fn parse_float(&mut self) -> Value {
        if let scanner::Token::Float(val) = self.input.current {
            self.input.get_next_token();
            return Value::Float(val);
        } else {
            self.input.get_next_token();
            return Value::Error;
        }
    }

    fn parse_parens(&mut self) -> Value {
        self.input.get_next_token(); // consume "("
        let value: Value = self.parse_expression();
        self.input.get_next_token(); // consume ")"
        return value;
    }

    /* for debugging/etc - simply repeats tokens back to user, 1 per line
    */
    #[allow(dead_code)]
    fn repeat_tokens(&mut self) {
        loop {
            let token = self.input.get_next_token();
            println!("{:?}", token);
        }
    }
}

fn main() {
    let mut parser = Parser {
        input: scanner::Input {
            stream: String::from(""),
            current: scanner::Token::NewLine,
            history: VecDeque::new(),
        },
        state: State {
            vars: HashMap::new(),
        },
    };

    // recursive descent parse
    parser.parse_program();
}
