extern crate regex;

use regex::Regex;

// Scanner/lexer

enum Token {
    // parsing logistics
    NewLine,
    WhiteSpace(i32),
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
    // if string is blank, get user input, set it to stream
    if stream == "" {
        io::stdin().read_line(&mut stream)
            .expect("Failed to read line");
    }

    // regex options
    // TODO figure out how to prevent repeated compiling
    let re_newline = Regex::new(r"^\n").unwrap();
    let re_whitespace = Regex::new(r"^[ ]+").unwrap();
    let re_list = Regex::new(r"^list[\n ]").unwrap();
    let re_del = Regex::new(r"^del[\n ]").unwrap();
    let re_exit = Regex::new(r"^exit[\n ]").unwrap();
    let re_none = Regex::new(r"^None[\n ]").unwrap();
    let re_variable = Regex::new(r"^[A-z][A-z0-9]*").unwrap(); 
    let re_plus = Regex::new(r"^\+").unwrap();
    let re_minus = Regex::new(r"^-").unwrap();
    let re_exponent = Regex::new(r"^\*\*").unwrap();
    let re_multiply = Regex::new(r"^\*").unwrap();
    let re_divide = Regex::new(r"^/").unwrap();
    let re_leftparen = Regex::new(r"^\(").unwrap();
    let re_rightparen = Regex::new(r"^\)").unwrap();
    let re_eq = Regex::new(r"^=").unwrap();
    let re_float = Regex::new(r"^[0-9]+\.[0-9]*").unwrap();
    let re_int = Regex::new(r"^[0-9]+").unwrap();

    if let Some(x) = check_match(&stream, re_newline) {
        return (String::from(&stream[x.len()..]), Token::NewLine);
    } else if let Some(x) = check_match(&stream, re_whitespace) {
        return (String::from(&stream[x.len()..]), Token::WhiteSpace(x.len() as i32));
    } else if let Some(x) = check_match(&stream, re_list) {
        return (String::from(&stream[x.len()..]), Token::List);
    } else if let Some(x) = check_match(&stream, re_del) {
        return (String::from(&stream[x.len()..]), Token::Del);
    } else if let Some(x) = check_match(&stream, re_exit) {
        return (String::from(&stream[x.len()..]), Token::Exit);
    } else if let Some(x) = check_match(&stream, re_none) {
        return (String::from(&stream[x.len()..]), Token::NoneT);
    } else if let Some(x) = check_match(&stream, re_variable) {
        return (String::from(&stream[x.len()..]), Token::Variable(String::from(x)));
    } else if let Some(x) = check_match(&stream, re_plus) {
        return (String::from(&stream[x.len()..]), Token::Plus);
    } else if let Some(x) = check_match(&stream, re_minus) {
        return (String::from(&stream[x.len()..]), Token::Minus);
    } else if let Some(x) = check_match(&stream, re_exponent) {
        return (String::from(&stream[x.len()..]), Token::Exponent);
    } else if let Some(x) = check_match(&stream, re_multiply) {
        return (String::from(&stream[x.len()..]), Token::Multiply);
    } else if let Some(x) = check_match(&stream, re_divide) {
        return (String::from(&stream[x.len()..]), Token::Divide);
    } else if let Some(x) = check_match(&stream, re_leftparen) {
        return (String::from(&stream[x.len()..]), Token::LeftParen);
    } else if let Some(x) = check_match(&stream, re_rightparen) {
        return (String::from(&stream[x.len()..]), Token::RightParen);
    } else if let Some(x) = check_match(&stream, re_eq) {
        return (String::from(&stream[x.len()..]), Token::Equals);
    } else if let Some(x) = check_match(&stream, re_float) {
        return (String::from(&stream[x.len()..]), Token::Float(x.parse().unwrap()));
    } else if let Some(x) = check_match(&stream, re_int) {
        return (String::from(&stream[x.len()..]), Token::Int(x.parse().unwrap()));
    } else {
        return (String::new(), Token::Error)
    }
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

