pub mod scanner;

fn main() {
    //// scanner
    // initialize stream
    let mut pair = scanner::get_next_token("".to_string());
    let mut stream = pair.0;
    let mut token = &pair.1;

    // continually retrieve tokens
    loop {
        match token {
            scanner::Token::Exit => break,
            _ => pair = scanner::get_next_token(stream)
        }
        stream = pair.0;
        token = &pair.1;
        // update token values
        match token {
            scanner::Token::WhiteSpace(_len) => {
                // do something
            },
            scanner::Token::Float(_val) => {
                // do something
            },
            scanner::Token::Int(_val) => {
                // do something
            },
            scanner::Token::Variable(_name) => {
                // do something
            },
            // TODO debugging purposes
            _ => {
                // do something
            }
        };
        scanner::print_token(token);
    }
}
