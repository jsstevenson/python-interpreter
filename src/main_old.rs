pub mod scanner;
use std::collections::{VecDeque, HashMap};

#[derive(Clone, Copy, Debug)]
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
 * Each entry is a mapping between a variable name and Data, containing info
 * concerning its value
 */
struct State {
    vars: HashMap<String, Data>
}


/* program ::= statement | program statement
 *
 */
fn parse_program(input: &mut scanner::Input, state: &mut State) {
    loop {
        // update current token
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
        parse_statement(input, state);
    }
}

/* statement ::= expr | id = expr | clear id | list | quit | exit | epsilon
 *
 */
fn parse_statement(input: &mut scanner::Input, state: &mut State) -> Value {
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

fn parse_exit(_input: &mut scanner::Input, _state: &mut State) -> Value {
    return Value::Error;
}

/* use for debugging for now - will remove
 */
fn parse_list(_input: &mut scanner::Input, _state: &mut State) -> Value {
    return Value::Error;
}

fn parse_clear(_input: &mut scanner::Input, _state: &mut State) -> Value {
    return Value::Error;
}

/* Parse newline
 */
fn parse_newline(input: &mut scanner::Input, _state: &mut State) -> Value {
    // consume token
    input.get_next_token();
    return Value::Terminator;
}

fn parse_assign(input: &mut scanner::Input, state: &mut State) -> Value {
    let input_copy = input.clone();
    if let scanner::Token::Variable(name) = input_copy.current {
        input.get_next_token();
        let data = Data {
            type_meta: Type::Int, // TODO placeholder
            value_meta: parse_statement(input, state)
        };
        state.vars.insert(String::from(name), data.clone());
        return data.value_meta;
    } else {
        return Value::Error;
    }
}


/* expr ::= term | expr + term | expr - term
 *
 */
fn parse_expr(input: &mut scanner::Input, state: &mut State) -> Value {
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
fn parse_term(input: &mut scanner::Input, state: &mut State) -> Value {
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
fn parse_power(input: &mut scanner::Input, state: &mut State) -> Value {
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

/* factor ::= var | number | (exp) |  -exp
 */
fn parse_factor(input: &mut scanner::Input, state: &mut State) -> Value {
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

fn parse_var(input: &mut scanner::Input, state: &mut State) -> Value {
    let input_copy = input.clone();
    if let scanner::Token::Variable(var_name) = &input_copy.current {
        match state.vars.get(var_name) {
            None => {
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

fn parse_lparen(input: &mut scanner::Input, state: &mut State) -> Value {
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
    let mut state = State {
        vars: HashMap::new()
    };

    // recursive descent parse
    parse_program(&mut input, &mut state);
}
