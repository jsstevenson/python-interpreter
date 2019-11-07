use regex::Regex;
use std::io;
// Scanner
// Scanner struct contains stream

struct Scanner {
    stream: String
}

enum Token {
    // logistical
    newline,
    // keywords
    clear,
    list,
    quit,
    exit,
    // operators
    add,
    subt,
    expn,
    mult,
    div,
    // organizing, etc
    l_paren,
    r_paren,
    eq,
    // numbers
    int,
    float,
    // variables
    vars(String)
}

impl Scanner {
    // TODO: allow for optional initial stream arg
    fn initialize(&self} {
        let mut input = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        self.stream = input;
    }

    fn next_token(&self) -> Token {
        
    }
}

fn scanner_interface() {

}

fn main() {
}
