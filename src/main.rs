pub mod scanner;
use std::collections::VecDeque;

fn main() {
    // initialize stream, history
    let mut input = scanner::Input {
        stream: String::from(""),
        history: VecDeque::new()
    };

    let mut token = scanner::Token::NewLine;

    // recursive descent parse
    loop {
        match token {
            scanner::Token::Exit => break,
            _ => {
                // put next_token wrapping in here?
                if input.history.is_empty() {
                    token = input.get_next_token();
                } else {
                    // get from history
                    token = input.history.pop_front().unwrap();
                }
            }
        }
        // destructure
        scanner::print_token(&token); // for debugging
    }
}
