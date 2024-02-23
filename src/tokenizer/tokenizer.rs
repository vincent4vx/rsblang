use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::tokenizer::Token;

enum TokenizerState {
    Empty,
    AmbiguousOperator,
    SymbolOrKeyword,
    Comment,
    Char,
    String,
    Numeric,
}

pub struct Tokenizer {
    tokens: Vec<Token>,
    buffer: String,
    state: TokenizerState,
}

#[derive(Debug, Clone)]
pub struct TokenError {
    message: &'static str
    // @todo add token positions
}

impl TokenError {
    fn new(message: &'static str) -> TokenError {
        TokenError {
            message,
        }
    }
}

impl Display for TokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenError: {}", self.message)
    }
}

impl Error for TokenError {

}

type Result<T> = std::result::Result<T, TokenError>;

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            tokens: Vec::new(),
            buffer: String::new(),
            state: TokenizerState::Empty,
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn finalize(&mut self) -> Result<()> {
        return self.save_token();
    }

    pub fn push(&mut self, c: char) -> Result<()> {
        match self.state {
            TokenizerState::Empty => {
                match c {
                    c if c.is_whitespace() => Ok(()),
                    c if c.is_numeric() => {
                        self.state = TokenizerState::Numeric;
                        self.buffer.push(c);

                        Ok(())
                    }
                    '"' => {
                        self.state = TokenizerState::String;
                        Ok(())
                    }
                    '\'' => {
                        self.state = TokenizerState::Char;
                        Ok(())
                    }
                    '(' => Ok(self.tokens.push(Token::OpeningParenthesis)),
                    ')' => Ok(self.tokens.push(Token::ClosingParenthesis)),
                    '[' => Ok(self.tokens.push(Token::OpeningBracket)),
                    ']' => Ok(self.tokens.push(Token::ClosingBracket)),
                    '{' => Ok(self.tokens.push(Token::OpeningBrace)),
                    '}' => Ok(self.tokens.push(Token::ClosingBrace)),
                    ';' => Ok(self.tokens.push(Token::EndOfStatement)),
                    ','|':'|'|'|'&'|'%'|'*'|'?' => Ok(self.tokens.push(Token::Operator(String::from(c)))),
                    '+'|'-'|'/'|'!'|'='|'>'|'<' => {
                        self.state = TokenizerState::AmbiguousOperator;
                        self.buffer.push(c);

                        Ok(())
                    }
                    _ => {
                        self.state = TokenizerState::SymbolOrKeyword;
                        self.buffer.push(c);

                        Ok(())
                    }
                }
            }
            TokenizerState::AmbiguousOperator => {
                match c {
                    c if c.is_whitespace() => self.save_token(),
                    c if c.is_numeric() => {
                        match self.save_token() {
                            Ok(_) => {
                                self.state = TokenizerState::Numeric;
                                self.buffer.push(c);

                                Ok(())
                            }
                            Err(e) => Err(e)
                        }
                    }

                    // "double" operators
                    '+'|'-'|'>'|'<'|'=' if self.buffer.ends_with(c) => {
                        self.buffer.push(c);

                        self.save_token()
                    }

                    // comment "/*"
                    '*' if self.buffer.ends_with('/') => {
                        self.state = TokenizerState::Comment;
                        self.buffer.clear();

                        Ok(())
                    }

                    // inequalities
                    '=' if ['!', '>', '<'].contains(&self.buffer.chars().next().unwrap()) => { // @todo do not use unwrap
                        self.buffer.push(c);

                        self.save_token()
                    }

                    '"' => {
                        match self.save_token() {
                            Ok(_) => Ok(self.state = TokenizerState::String),
                            Err(e) => Err(e)
                        }
                    }
                    '\'' => {
                        match self.save_token() {
                            Ok(_) => Ok(self.state = TokenizerState::Char),
                            Err(e) => Err(e)
                        }
                    }
                    '(' => self.save_and_push(Token::OpeningParenthesis),
                    ')' => self.save_and_push(Token::ClosingParenthesis),
                    '[' => self.save_and_push(Token::OpeningBracket),
                    ']' => self.save_and_push(Token::ClosingBracket),
                    '{' => self.save_and_push(Token::OpeningBrace),
                    '}' => self.save_and_push(Token::ClosingBrace),
                    ';' => self.save_and_push(Token::EndOfStatement),
                    ','|':'|'|'|'&'|'%'|'?'|'!'|'*'|'>'|'<'|'+'|'-'|'/'|'=' => self.save_and_push(Token::Operator(String::from(c))),
                    _ => {
                        self.save_token()
                            .and_then(|_| self.push(c))
                    }
                }
            }
            TokenizerState::SymbolOrKeyword => {
                match c {
                    c if c.is_whitespace() => self.save_token(),

                    '(' => self.save_and_push(Token::OpeningParenthesis),
                    ')' => self.save_and_push(Token::ClosingParenthesis),
                    '[' => self.save_and_push(Token::OpeningBracket),
                    ']' => self.save_and_push(Token::ClosingBracket),
                    '{' => self.save_and_push(Token::OpeningBrace),
                    '}' => self.save_and_push(Token::ClosingBrace),
                    ';' => self.save_and_push(Token::EndOfStatement),

                    ','|':'|'|'|'&'|'%'|'?'|'!'|'*'|'>'|'<'|'+'|'-'|'/'|'=' => {
                        self.save_token()
                            .and_then(|_| self.push(c)) // push from initial state
                    },

                    _ => {
                        self.buffer.push(c);

                        Ok(())
                    }
                }
            }
            TokenizerState::Comment => {
                if c == '/' && self.buffer.pop().is_some_and(|o| o == '*') {
                    self.reset_state();
                }

                if c == '*' {
                    self.buffer.push(c);
                }

                Ok(())
            }
            TokenizerState::Char => {
                if c == '\'' {
                    return self.save_token();
                }

                // @todo parse meta char

                self.buffer.push(c);

                Ok(())
            }
            TokenizerState::String => {
                if c == '"' {
                    return self.save_token();
                }

                // @todo parse meta char

                self.buffer.push(c);

                Ok(())
            }
            TokenizerState::Numeric => {
                if c.is_numeric() {
                    self.buffer.push(c);

                    return Ok(());
                }

                self.save_token()
                    .and_then(|_| self.push(c))
            }
        }
    }

    fn save_and_push(&mut self, token: Token) -> Result<()> {
        match self.save_token() {
            Ok(_) => {
                self.tokens.push(token);

                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    fn save_token(&mut self) -> Result<()> {
        match self.state {
            TokenizerState::Empty => {}
            TokenizerState::AmbiguousOperator => self.tokens.push(Token::Operator(self.buffer.clone())),
            TokenizerState::SymbolOrKeyword => {
                match self.buffer.as_str() {
                    "auto" => self.tokens.push(Token::Auto),
                    "extrn" => self.tokens.push(Token::Extern),
                    "case" => self.tokens.push(Token::Case),
                    "if" => self.tokens.push(Token::If),
                    "else" => self.tokens.push(Token::Else),
                    "while" => self.tokens.push(Token::While),
                    "switch" => self.tokens.push(Token::Switch),
                    "goto" => self.tokens.push(Token::Goto),
                    "return" => self.tokens.push(Token::Return),
                    _ => self.tokens.push(Token::Symbol(self.buffer.clone())),
                }
            }
            TokenizerState::Comment => {}
            TokenizerState::Char => {
                let mut chars: [char; 4] = ['\0'; 4];

                if self.buffer.len() < 1 || self.buffer.len() > 4 {
                    return Err(TokenError::new("Invalid character size : must be between 1 and 4"));
                }

                self.buffer.chars().enumerate().for_each(|(i, c)| chars[i] = c);
                self.tokens.push(Token::Char(chars));
            }
            TokenizerState::String => self.tokens.push(Token::String(self.buffer.clone())),
            TokenizerState::Numeric => {
                match self.buffer.parse() {
                    Ok(i) => self.tokens.push(Token::Integer(i)),
                    Err(_) => return Err(TokenError::new("Error during parsing number token"))
                }
            }
        }

        self.reset_state();

        Ok(())
    }

    fn reset_state(&mut self) {
        self.state = TokenizerState::Empty;
        self.buffer.clear();
    }
}
