pub use tokenizer::Tokenizer;
pub use tokens::Token;

mod tokens;
mod tokenizer;
mod util;

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::tokenizer::Token;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn from_file_success() {
        let tokenizer = Tokenizer::from_file(Path::new("example/printn.b")).unwrap();
        let tokens = tokenizer.tokens();

        assert_eq!(38, tokens.len());

        assert_eq!(Token::Symbol(String::from("printn")), tokens[0]);
        assert_eq!(Token::OpeningParenthesis, tokens[1]);
        assert_eq!(Token::Symbol(String::from("n")), tokens[2]);
        assert_eq!(Token::Operator(String::from(",")), tokens[3]);
        assert_eq!(Token::Symbol(String::from("b")), tokens[4]);
        assert_eq!(Token::ClosingParenthesis, tokens[5]);
        assert_eq!(Token::OpeningBrace, tokens[6]);
        assert_eq!(Token::Extern, tokens[7]);
        assert_eq!(Token::Symbol(String::from("putchar")), tokens[8]);
        assert_eq!(Token::EndOfStatement, tokens[9]);
        assert_eq!(Token::Auto, tokens[10]);
        assert_eq!(Token::Symbol(String::from("a")), tokens[11]);
        assert_eq!(Token::EndOfStatement, tokens[12]);
        assert_eq!(Token::If, tokens[13]);
        assert_eq!(Token::OpeningParenthesis, tokens[14]);
        assert_eq!(Token::Symbol(String::from("a")), tokens[15]);
        assert_eq!(Token::Operator(String::from("=")), tokens[16]);
        assert_eq!(Token::Symbol(String::from("n")), tokens[17]);
        assert_eq!(Token::Operator(String::from("/")), tokens[18]);
        assert_eq!(Token::Symbol(String::from("b")), tokens[19]);
        assert_eq!(Token::ClosingParenthesis, tokens[20]);
        assert_eq!(Token::Symbol(String::from("printn")), tokens[21]);
        assert_eq!(Token::OpeningParenthesis, tokens[22]);
        assert_eq!(Token::Symbol(String::from("a")), tokens[23]);
        assert_eq!(Token::Operator(String::from(",")), tokens[24]);
        assert_eq!(Token::Symbol(String::from("b")), tokens[25]);
        assert_eq!(Token::ClosingParenthesis, tokens[26]);
        assert_eq!(Token::EndOfStatement, tokens[27]);
        assert_eq!(Token::Symbol(String::from("putchar")), tokens[28]);
        assert_eq!(Token::OpeningParenthesis, tokens[29]);
        assert_eq!(Token::Symbol(String::from("n")), tokens[30]);
        assert_eq!(Token::Operator(String::from("%")), tokens[31]);
        assert_eq!(Token::Symbol(String::from("b")), tokens[32]);
        assert_eq!(Token::Operator(String::from("+")), tokens[33]);
        assert_eq!(Token::Char(['0', '\0', '\0', '\0']), tokens[34]);
        assert_eq!(Token::ClosingParenthesis, tokens[35]);
        assert_eq!(Token::EndOfStatement, tokens[36]);
        assert_eq!(Token::ClosingBrace, tokens[37]);
    }

    #[test]
    fn from_file_not_found() {
        match Tokenizer::from_file(Path::new("not_found")) {
            Ok(_) => assert!(false, "An error should be returned"),
            Err(e) => assert_eq!("No such file or directory (os error 2)", e.to_string()),
        }
    }
}
