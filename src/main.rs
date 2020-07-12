mod scanner;
mod parser;
// use std::collections::{HashMap, VecDeque};

fn main() {
    let mut parser = parser::Parser::build_new();

    // recursive descent parse
    parser.parse_program();
}
