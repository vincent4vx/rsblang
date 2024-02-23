use std::path::Path;

use crate::tokenizer::Tokenizer;

mod tokenizer;

fn main() {
    let tokenizer = match Tokenizer::from_file(Path::new("example/printn.b")) {
        Ok(t) => t,
        Err(e) => panic!("Cannot parse file : {}", e)
    };

    for token in tokenizer.tokens() {
        println!("{:?}", token);
    }
}
