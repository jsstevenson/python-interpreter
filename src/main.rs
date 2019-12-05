extern crate regex;

use regex::Regex;
use std::io;

// Scanner/lexer

enum Token {
    // parsing logistics
    NewLine,
    WhiteSpace(u64),
    // keywords
    List,
    Del,
    Exit,
    NoneT,
    // operators
    Plus,
    Minus,
    Exponent,
    Multiply,
    Divide,
    // organizing, etc
    LeftParen,
    RightParen,
    Equals,
    // numbers
    Float(f64),
    Int(u64),
    // variables
    Variable(String),
    // errors
    Error
}

/* Returns result of attempt to match given regex pattern to the stream
 * If no match, returns None. Otherwise, returns a string slice of the match
 * from the stream, as a Some().
 */
fn check_match(stream: &str, re: Regex) -> Option<&str> {
    if re.is_match(stream) {
        return Some(re.find(stream).unwrap().as_str());
    } else {
        return None;
    }
}

/* General retriever of next token.
 * Takes stream, a String containing line(s) of input, grabs the longest form
 * of the first token it finds, and returns a tuple of the String sans that
 * token as well as the corresponding Token struct.
 *
 * General notes:
 *  - WhiteSpace should store length (for determining scope)
 *  - TODO Need to work out how to raise error, and store error type
 */
fn get_next_token(mut stream: String) -> (String, Token) {
    let patterns_map = vec![
        (Regex::new(r"^\n").unwrap(), Token::NewLine),
        (Regex::new(r"^[ ]+").unwrap(), Token::WhiteSpace(0)),
        (Regex::new(r"^list").unwrap(), Token::List),
        (Regex::new(r"^del").unwrap(), Token::Del),
        (Regex::new(r"^exit").unwrap(), Token::Exit),
        (Regex::new(r"^None").unwrap(), Token::NoneT),
        (Regex::new(r"^\+").unwrap(), Token::Plus),
        (Regex::new(r"^-").unwrap(), Token::Minus),
        (Regex::new(r"^\*\*").unwrap(), Token::Exponent),
        (Regex::new(r"^\*").unwrap(), Token::Multiply),
        (Regex::new(r"^/").unwrap(), Token::Divide),
        (Regex::new(r"^\(").unwrap(), Token::LeftParen),
        (Regex::new(r"^\)").unwrap(), Token::RightParen),
        (Regex::new(r"^=").unwrap(), Token::Equals),
        (Regex::new(r"^[0-9]+\.[0-9]*").unwrap(), Token::Float(0.0)),
        (Regex::new(r"^[0-9]+").unwrap(), Token::Int(0)),
        (Regex::new(r"^[A-z][A-z0-9]*").unwrap(), Token::Variable(String::new())),
        // TODO error handling here
    ];

    // if string is blank, get user input, set it to stream
    if stream == "" {
        io::stdin().read_line(&mut stream)
            .expect("Failed to read line");
    }

    // iterate through matches
    for (re, token) in patterns_map {
        if re.is_match(&stream) {
            // remove token from stream
            stream = stream[re.find(&stream).unwrap().end()..].to_string();
            return (stream, token)
        }
    }

    // if no matches, return error
    return (String::new(), Token::Error)
}

/* print_token - debugging utility. Prints type of supplied Token, and value
 * where relevant.
 */
fn print_token(token: &Token) {
    match token {
        Token::NewLine => println!("newline"),
        Token::WhiteSpace(len) => println!("whitespace length: {}", len),
        Token::List => println!("list"),
        Token::Del => println!("Delete"),
        Token::Exit => println!("Exit"),
        Token::NoneT => println!("NoneType"),
        Token::Plus => println!("Plus"),
        Token::Minus => println!("Minus"),
        Token::Exponent => println!("Exponent"),
        Token::Multiply => println!("Multiply"),
        Token::Divide => println!("Divide"),
        Token::LeftParen => println!("Left Paren"),
        Token::RightParen => println!("Right Paren"),
        Token::Equals => println!("Equals"),
        Token::Float(val) => println!("Float: {}", val),
        Token::Int(val) => println!("Int: {}", val),
        Token::Variable(name) => println!("Variable name: {}", name),
        Token::Error => println!("Error"),
    }
}

fn main() {
    //// scanner
    // initialize stream
    let mut pair = get_next_token("".to_string());
    let mut stream = pair.0;
    let mut token = &pair.1;

    // continually retrieve tokens
    loop {
        match token {
            Token::Exit => break,
            _ => pair = get_next_token(stream)
        }
        stream = pair.0;
        token = &pair.1;
        // update token values
        match token {
            Token::WhiteSpace(_len) => {
                // do something
            },
            Token::Float(_val) => {
                // do something
            },
            Token::Int(_val) => {
                // do something
            },
            Token::Variable(_name) => {
                // do something
            },
            _ => ()
        }
        // TODO debugging purposes
        print_token(token);
    }
}
