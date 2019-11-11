extern crate regex;

use regex::Regex;
use std::io;

// Scanner/lexer

enum Token {
    // parsing logistics
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
    Float(f64),
    Int(u64),
    // variables
    Variable(String),
    // errors
    Error
}

fn get_next_token(mut stream: String) -> (String, Token) {
    // TODO replace with match?
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
        (Regex::new(r"^=").unwrap(), Token::Equals),
        (Regex::new(r"^[0-9]+\.[0-9]*").unwrap(), Token::Float(0.0)),
        (Regex::new(r"^[0-9]+").unwrap(), Token::Int(0))
    ];

    // if string is blank, get user input, set it to stream
    if stream == "" {
        io::stdin().read_line(&mut stream)
            .expect("Failed to read line");
    }

    // iterate through matches
    for (re, token) in patterns_map {
        if re.is_match(&stream) {
            // modify stream
            stream = stream[re.find(&stream).unwrap().end()..].to_string();
            return (stream, token)
        }
    }

    // if no matches, return error
    return (String::new(), Token::Error)
}


fn main() {
    //// scanner
    // initialize stream
    let mut stream = String::from("");
    let mut token = Token::NoneT;

    // while stream exists:
    //  run stream against patterns
    //  retrieve regex match
    //  from match, get token
    //  from match, snip stream
    let temp = get_next_token(stream);
    stream = temp.0;
    token = temp.1;

}
