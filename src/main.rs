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

// General retriever of next token.
// Takes stream, a String containing line(s) of input, grabs the longest form
// of the first token it finds, and returns a tuple of the String sans that
// token as well as the corresponding Token struct
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
        (Regex::new(r"^[0-9]+").unwrap(), Token::Int(0)),
        (Regex::new(r"^[A-z\_][A-z0-9\_]*").unwrap(), Token::Variable(String::new()))
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

fn print_token(token: &Token) {
    match token {
        Token::NewLine => println!("newline"),
        Token::WhiteSpace => println!("whitespace length"),
        Token::List => println!("list"),
        _ => println!("other?")
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
        // TODO debugging
        print_token(token);
    }
}
