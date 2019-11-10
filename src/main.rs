extern crate regex;

use regex::Regex;
use std::io;

// Scanner/lexer

enum Token {
    // logistical
    NewLine,
    WhiteSpace,
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
    Int(u64),
    Float(f64),
    // variables
    Variable(String)
}

fn make_patterns_map() -> Vec<(Regex, Token)> {
    let patterns_map = vec![
        (Regex::new(r"^\n").unwrap(), Token::NewLine),
        (Regex::new(r"^[ ]+").unwrap(), Token::WhiteSpace),
        (Regex::new(r"^list").unwrap(), Token::List),
        (Regex::new(r"^del").unwrap(), Token::Del),
        (Regex::new(r"^exit").unwrap(), Token::Exit),
        (Regex::new(r"^None").unwrap(), Token::NoneT),
        (Regex::new(r"^+").unwrap(), Token::Plus),
        (Regex::new(r"^-").unwrap(), Token::Minus),
        (Regex::new(r"^\*\*").unwrap(), Token::Exponent),
        (Regex::new(r"^\*").unwrap(), Token::Multiply),
        (Regex::new(r"^/").unwrap(), Token::Divide),
        (Regex::new(r"^\(").unwrap(), Token::LeftParen),
        (Regex::new(r"^\)").unwrap(), Token::RightParen),
        (Regex::new(r"^=").unwrap(), Token::Equals)
    ];
    return patterns_map;
}

fn get_next_token(stream: String) -> (String, Token) {
    // TODO replace with match?
    if Regex
    return (
}

fn update_stream(Regex) {

}

fn main() {
    //// scanner
    // initialize stream
    let mut stream = String::from("");
    let patterns = make_patterns_map();
    let token = Token::NoneT;

    // while stream exists:
    //  run stream against patterns
    //  retrieve regex match
    //  from match, get token
    //  from match, snip stream

}
