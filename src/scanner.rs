extern crate regex;

use regex::Regex;
use std::collections::VecDeque;
use std::io::{stdin, stdout, Write}; // TODO double check necessary

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Token {
    SyntaxError,
    State,
    Exit,
    // values (WIP)
    Float(f64),
    Int(i64),
    // variables
    Variable(String),
    // misc - could easily move to terminals
    WhiteSpace(usize),
    // for parsing logistics
    NewLine,
    // operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    // organization
    Equals,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    SingleQuote,
    DoubeleQuote,
    Colon,
    // functions and classes, etc
    Def,
    Return,
    Lambda,
    With,
    As,
    Class,
    // assertions/tries
    Try,
    Except,
    Raise,
    // loops and iterations
    For,
    In,
    While,
    Continue,
    Pass,
    Break,
    Finally,
    Yield,
    // importation
    FromImport,
    Import,
    // Boolean and related
    If,
    Elif,
    Else,
    True,
    False,
    Is,
    And,
    Not,
    Or,
    // Misc
    NoneT,
    Async,
    Assert,
    Await,
    Del,
    Global,
    Nonlocal,
}

#[derive(Debug)]
pub struct RegexMatch {
    token: Token,
    token_len: usize,
}

/* Basic input struct. Holds current input remaining to be parsed, as well as
 * a basic history deque in case of lookahead
 */
#[derive(Debug)]
pub struct Input {
    pub stream: String,
    pub current: Token,
    pub history: VecDeque<Token>,
}

impl Input {
    /* for bailing out when errors arise
     */
    fn flush_line(&mut self) {
        self.stream = String::from("");
        self.history.clear();
    }

    /* Returns result of attempt to match given regex pattern to the stream.
     * If no match, returns None; otherwise, returns a string slice of the match
     * from the stream
     */
    fn check_match(stream: &str, re: Regex) -> Option<&str> {
        if re.is_match(stream) {
            return Some(re.find(stream).unwrap().as_str());
        } else {
            return None;
        }
    }

    pub fn re_match(&mut self) -> RegexMatch {
        // regex options
        let re_newline = Regex::new(r"^\n").unwrap();
        let re_whitespace = Regex::new(r"^[ ]+").unwrap();
        let re_del = Regex::new(r"^del[\n ]").unwrap();
        let re_exit = Regex::new(r"^exit[\n ]").unwrap();
        let re_state = Regex::new(r"^state[\n ]").unwrap();
        let re_none = Regex::new(r"^None[\n ]").unwrap();
        let re_variable = Regex::new(r"^[A-z][A-z0-9]*").unwrap();
        let re_plus = Regex::new(r"^\+").unwrap();
        let re_minus = Regex::new(r"^-").unwrap();
        let re_exponent = Regex::new(r"^\*\*").unwrap();
        let re_multiply = Regex::new(r"^\*").unwrap();
        let re_divide = Regex::new(r"^/").unwrap();
        let re_openparen = Regex::new(r"^\(").unwrap();
        let re_closeparen = Regex::new(r"^\)").unwrap();
        let re_eq = Regex::new(r"^=").unwrap();
        let re_float = Regex::new(r"^[0-9]+\.[0-9]*").unwrap();
        let re_int = Regex::new(r"^[0-9]+").unwrap();

        if let Some(_) = Input::check_match(&self.stream, re_newline) {
            return RegexMatch {
                token: Token::NewLine,
                token_len: 1,
            };
        } else if let Some(whitespace) = Input::check_match(&self.stream, re_whitespace) {
            let len: usize = whitespace.len();
            return RegexMatch {
                token: Token::WhiteSpace(len),
                token_len: len,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_del) {
            return RegexMatch {
                token: Token::NewLine,
                token_len: 3,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_exit) {
            return RegexMatch {
                token: Token::Exit,
                token_len: 4,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_state) {
            return RegexMatch {
                token: Token::State,
                token_len: 4,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_none) {
            return RegexMatch {
                token: Token::NoneT,
                token_len: 4,
            };
        } else if let Some(name) = Input::check_match(&self.stream, re_variable) {
            let name_clone = String::from(name.clone());
            let name_clone_len = name_clone.len();
            return RegexMatch {
                token: Token::Variable(name_clone),
                token_len: name_clone_len,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_plus) {
            return RegexMatch {
                token: Token::Plus,
                token_len: 1,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_minus) {
            return RegexMatch {
                token: Token::Minus,
                token_len: 1,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_exponent) {
            return RegexMatch {
                token: Token::Exponent,
                token_len: 2,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_multiply) {
            return RegexMatch {
                token: Token::Multiply,
                token_len: 1,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_divide) {
            return RegexMatch {
                token: Token::Divide,
                token_len: 1,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_openparen) {
            return RegexMatch {
                token: Token::OpenParen,
                token_len: 1,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_closeparen) {
            return RegexMatch {
                token: Token::CloseParen,
                token_len: 1,
            };
        } else if let Some(_) = Input::check_match(&self.stream, re_eq) {
            return RegexMatch {
                token: Token::Equals,
                token_len: 1,
            };
        } else if let Some(val) = Input::check_match(&self.stream, re_float) {
            let val_parsed = val.parse().unwrap();
            return RegexMatch {
                token: Token::Float(val_parsed),
                token_len: val.len(),
            };
        } else if let Some(val) = Input::check_match(&self.stream, re_int) {
            let val_parsed = val.parse().unwrap();
            return RegexMatch {
                token: Token::Int(val_parsed),
                token_len: val.len(),
            };
        } else {
            return RegexMatch {
                token: Token::SyntaxError,
                token_len: 0,
            };
        }
    }

    /* Look ahead - assists w/ parsing
     *
     * ignore_whitespace: if true, consumes + skips whitespace
     */
    pub fn look_ahead(&mut self, ignore_whitespace: bool) -> &Token {
        let mut next_token_match: RegexMatch = self.re_match();

        if ignore_whitespace {
            match next_token_match.token {
                Token::WhiteSpace(_) => {
                    self.stream = String::from(&self.stream[next_token_match.token_len..]);
                    next_token_match = self.re_match();
                },
                _ => (),
            }
        }

        // update current, stream
        match next_token_match.token {
            Token::SyntaxError => {
                self.history.push_back(next_token_match.token);
                self.flush_line();
            }
            _ => {
                self.history.push_back(next_token_match.token);
                self.stream = String::from(&self.stream[next_token_match.token_len..]);
            }
        };

        return &self.history.back().unwrap();
    }

    /* Get next token. Either pop from history queue, or consume next token
     * from input stream.
     *
     * skip_whitespace: if true, consume + skip whitespace
     *
     * Updates self.current and returns the new current token as a borrow
     */
    pub fn get_next_token(&mut self, skip_whitespace: bool) -> &Token {
        if !self.history.is_empty() {
            self.current = self.history.pop_front().unwrap();
            return &self.current;
        }

        // if stream blank, get user input
        if self.stream == "" {
            print!(">> ");
            stdout().flush().expect("Could not flush stdout");
            stdin()
                .read_line(&mut self.stream)
                .expect("Failed to read line");
        }

        // get first match
        let next_token_match: RegexMatch = self.re_match();

        // update current, stream
        match next_token_match.token {
            Token::SyntaxError => {
                self.current = next_token_match.token;
                self.flush_line();
            }
            _ => {
                self.stream = String::from(&self.stream[next_token_match.token_len..]);
                self.current = next_token_match.token;
            }
        }

        if skip_whitespace {
            match &self.current {
                Token::WhiteSpace(_) => {
                    self.get_next_token(false);
                },
                _ => ()
            }
        }

        return &self.current;
    }
}
