extern crate regex;

use std::collections::VecDeque;
use regex::Regex;
use std::io;
use std::io::Write;


pub enum Token {
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

pub struct Input {
    pub stream: String,
    pub history: VecDeque<Token>
}

impl Input {

    /* Look ahead to future
     * Helps with parsing
     * TODO make it work
     */
    fn look_ahead(&mut self) -> Token {
        /* in: current stream, history list
         * out: stream, history list (+ last item in history list???)
         */
        let token = self.get_next_token();
        self.history.push_back(token);
        return Token::Error;
    }

    /* Empty line/tokens if error arises
    */
    fn flush_line(&mut self) {
        self.stream = String::from("");
        self.history.clear();
    }

    /* Returns result of attempt to match given regex pattern to the stream
     * If no match, returns None. Otherwise, returns a string slice of the match
     * from the stream, as a Some().
     */
    pub fn check_match(stream: &str, re: Regex) -> Option<&str> {
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
    pub fn get_next_token(&mut self) -> Token {
        // if string is blank, get user input, set it to stream
        if self.stream == "" {
            print!(">>> ");
            io::stdout().flush().expect("Could not flush stdout");
            io::stdin().read_line(&mut self.stream)
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

        if let Some(x) = Input::check_match(&self.stream, re_newline) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::NewLine
        } else if let Some(x) = Input::check_match(&self.stream, re_whitespace) {
            let val: i32 = x.len() as i32;
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::WhiteSpace(val)
        } else if let Some(x) = Input::check_match(&self.stream, re_list) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::List;
        } else if let Some(x) = Input::check_match(&self.stream, re_del) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Del;
        } else if let Some(x) = Input::check_match(&self.stream, re_exit) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Exit;
        } else if let Some(x) = Input::check_match(&self.stream, re_none) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::NoneT;
        } else if let Some(x) = Input::check_match(&self.stream, re_variable) {
            let val = String::from(x.clone());
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Variable(val);
        } else if let Some(x) = Input::check_match(&self.stream, re_plus) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Plus;
        } else if let Some(x) = Input::check_match(&self.stream, re_minus) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Minus;
        } else if let Some(x) = Input::check_match(&self.stream, re_exponent) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Exponent;
        } else if let Some(x) = Input::check_match(&self.stream, re_multiply) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Multiply;
        } else if let Some(x) = Input::check_match(&self.stream, re_divide) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Divide;
        } else if let Some(x) = Input::check_match(&self.stream, re_leftparen) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::LeftParen;
        } else if let Some(x) = Input::check_match(&self.stream, re_rightparen) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::RightParen;
        } else if let Some(x) = Input::check_match(&self.stream, re_eq) {
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Equals;
        } else if let Some(x) = Input::check_match(&self.stream, re_float) {
            let val = x.parse().unwrap();
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Float(val);
        } else if let Some(x) = Input::check_match(&self.stream, re_int) {
            let val = x.parse().unwrap();
            self.stream = String::from(&self.stream[x.len()..]);
            return Token::Int(val);
        } else {
            self.stream = String::new();
            return Token::Error;
        }
    }
}



/* print_token - debugging utility. Prints type of supplied Token, and value
 * where relevant.
 */
pub fn print_token(token: &Token) {
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
