use std::path::Path;
use crate::parser::Parser;

use crate::tokenizer::Tokenizer;

mod tokenizer;
mod parser;

fn main() {
    let tokenizer = match Tokenizer::from_file(Path::new("example/printn.b")) {
        Ok(t) => t,
        Err(e) => panic!("Cannot parse file : {}", e)
    };

    Parser::parse(tokenizer.tokens())
}
