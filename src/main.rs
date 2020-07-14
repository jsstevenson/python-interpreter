mod scanner;
mod parser;

fn main() {
    let mut parser = parser::Parser::build_new();

    // recursive descent parse
    parser.parse_program();
}
