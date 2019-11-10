extern crate regex;

use regex::Regex;
use std::io;
// Scanner
// Scanner struct contains stream

enum Token {
    // logistical
    NewLine,
    // keywords
    List,
    Del,
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
    Int(u64),
    Float(f64),
    // variables
    Variable(String)
}

fn initialize_stream(stream: &String) -> &String {
    return stream;
}

fn make_patterns_map() -> Vec<(Regex, Token)> {
    let patterns_map = vec![
        (Regex::new(r"^\n").unwrap(), Token::NewLine),
        (Regex::new(r"^list").unwrap(), Token::List),
        (Regex::new(r"^del").unwrap(), Token::Del),
        (Regex::new(r"^+").unwrap(), Token::Plus),
        (Regex::new(r"^-").unwrap(), Token::Minus),
        (Regex::new(r"^\*\*").unwrap(), Token::Exponent),
        (Regex::new(r"^\*").unwrap(), Token::Multiply),
        (Regex::new(r"^/").unwrap(), Token::Divide),
        (Regex::new(r"^\(").unwrap(), Token::LeftParen),
        (Regex::new(r"^\)").unwrap(), Token::RightParen),
        (Regex::new(r"^=").unwrap(), Token::Equals),

    ];
    return patterns_map;
}

fn main() {
}
