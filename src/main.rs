pub mod scanner;
use std::collections::{VecDeque, HashMap};

#[derive(Clone, Debug)]
enum Type {
    Int,
    Float,
    Str,
    Var
}

#[derive(Clone, Debug)]
enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Terminator,
    Error
}

/* Holds type and value of a given object/whatever.
 * Galaxy brain idea: "Value" shouldn't be an enum, but should be some raw
 * data, and "Type" should hold information for interpreting that data.
 * IE, value should just be some raw bits, and value tells us how to read them
 */
#[derive(Clone, Debug)]
struct Data {
    type_meta: Type,
    value_meta: Value
}

/* Holds environment variables
 * Need to think about nested scopes
 */
struct State {
    vars: HashMap<String, Data>
}

/* statement ::= expr | id = expr | clear id | list | quit | exit | epsilon
 *
 */
fn parse_statement(input: &scanner::Input, state: &State) -> Value {
    match input.current {
        scanner::Token::Exit => parse_exit(input, state),
        scanner::Token::List => parse_list(input, state),
        scanner::Token::Del => parse_clear(input, state),
        scanner::Token::NewLine => parse_newline(input, state),
        _ => {
            match input.look_ahead() {
                scanner::Token::Equals => parse_assign(input, state),
                _ => parse_expr(input, state)
            }
        }
    }
}

fn parse_exit(_input: &scanner::Input, _state: &State) -> Value {
    // TODO think about this
    return Value::Error;
}

fn parse_list(_input: &scanner::Input, _state: &State) -> Value {
    // TODO probably remove
    return Value::Error;
}

fn parse_clear(_input: &scanner::Input, _state: &State) -> Value {
    // TODO think about this
    return Value::Error;
}

/* Parse newline
 * TODO
 *  * Figure out how to deal w/ multiline functions/statements/expressions
 */
fn parse_newline(input: &scanner::Input, _state: &State) -> Value {
    // consume token
    input.get_next_token();
    return Value::Terminator;
}

fn parse_assign(input: &scanner::Input, state: &State) -> Value {
    if let scanner::Token::Variable(name) = &input.current {
        input.get_next_token();
        let data = Data {
            type_meta: Type::Int, // TODO gotta update
            value_meta: parse_statement(input, state)
        };
        state.vars.insert(String::from(name), data.clone());;
        return data.value_meta;
    } else {
        return Value::Error;
    }
}


/* expr ::= term | expr + term | expr - term
 *
 */
fn parse_expr(input: &scanner::Input, state: &State) -> Value {
    let mut return_value = parse_term(input, state);
    loop {
        match input.current {
            scanner::Token::Plus => {
                // consume operator token
                input.get_next_token();
                // parse next term
                if let Value::Int(val_int) = return_value {
                    if let Value::Int(val_parsed) = parse_term(input, state) {
                        return_value = Value::Int(val_int + val_parsed);
                    }
                }
            },
            scanner::Token::Minus=> {
                // consume operator token
                input.get_next_token();
                // parse next term
                if let Value::Int(val_int) = return_value {
                    if let Value::Int(val_parsed) = parse_term(input, state) {
                        return_value = Value::Int(val_int - val_parsed);
                    }
                }
            },
            _ => break
        }
    }
    return return_value;
}

/* term ::= power | term * power | term / power
 *
 */
fn parse_term(input: &scanner::Input, state: &State) -> Value {
    let mut return_value = parse_power(input, state);
    loop {
        match input.current {
            scanner::Token::Multiply => {
                // consume operator token
                input.get_next_token();
                // parse next term
                if let Value::Int(val_int) = return_value {
                    if let Value::Int(val_parsed) = parse_power(input, state) {

                        return_value = Value::Int(val_int * val_parsed);
                    }
                }
            },
            scanner::Token::Divide => {
                // consume operator token
                input.get_next_token();
                // parse next term
                if let Value::Int(val_int) = return_value {
                    if let Value::Int(val_parsed) = parse_power(input, state) {
                        return_value = Value::Int(val_int / val_parsed);
                    }
                }
            },
            _ => break
        }
    }
    return return_value;
}

/* power ::= factor | factor ** power
 * TODO non-int values... need some kind of abstraction
 */
fn parse_power(input: &scanner::Input, state: &State) -> Value {
    let mut term1 = parse_factor(input, state);
    match input.current {
        scanner::Token::Exponent => {
            // process exponent token
            input.get_next_token();
            // mutate return Value
            if let Value::Int(base) = term1 {
            if let Value::Int(exp) = parse_factor(input, state) {
            term1 = Value::Int(base.pow(exp as u32)); // TODO unexpected casting?
            };
            };
        },
        _ => ()
    }
    return term1;
}

/* factor ::= id | number | (exp) | sqrt(exp) | -exp
 * TODO sqrt, negative numbers
 */
fn parse_factor(input: &scanner::Input, state: &State) -> Value {
    match input.current {
        scanner::Token::Variable(_) => return parse_var(input, state),
        scanner::Token::Int(_) => return parse_int(input, state),
        scanner::Token::Float(_) => return parse_float(input, state),
        scanner::Token::LeftParen => return parse_lparen(input, state),
        _ => {
            println!("error (factor)");
            return Value::Error;
        },
    }
}

fn parse_var(input: &scanner::Input, state: &State) -> Value {
    if let scanner::Token::Variable(var_name) = input.current {
    match state.vars.get(&var_name) {
        None => {
            // TODO raise NameError
            input.get_next_token();
            return Value::Error;
        },
        Some(data) => {
            input.get_next_token();
            return data.value_meta;
        }
    };
    } else {
        return Value::Error;
    }
}

fn parse_int(input: &scanner::Input, _state: &State) -> Value {
    if let scanner::Token::Int(val) = input.current {
        return Value::Int(val);
    } else {
        return Value::Error;
    }
}

fn parse_float(input: &scanner::Input, _state: &State) -> Value {
    if let scanner::Token::Float(val) = input.current {
        return Value::Float(val);
    } else {
        return Value::Error;
    }
}

fn parse_lparen(input: &scanner::Input, state: &State) -> Value {
    // skip "("
    input.get_next_token();
    let value = parse_expr(input, state);
    // skip ")"
    input.get_next_token();
    return value;
}

fn main() {
    // initialize stream, history
    let mut input = scanner::Input {
        stream: String::from(""),
        current: scanner::Token::NewLine,
        history: VecDeque::new()
    };


    // recursive descent parse
    // program ::= statement | program statement
    loop {
        match input.current {
            scanner::Token::Exit => break,
            _ => {
                // get next token
                if input.history.is_empty() {
                    input.get_next_token();
                } else {
                    // get from history
                    input.current = input.history.pop_front().unwrap();
                }
            }
        }
        println!("{:?}", &input.current); // for debugging

        // parse
    }
}
