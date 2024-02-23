use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::Token::Char;

/**
 * All units of the code
 */
#[derive(Debug, Clone)]
enum Token {
    /**
     * Represent any symbol, like function name,
     * variable name, etc...
     * Takes as parameter the name
     *
     * Note: symbol are distinct from keyword, where each keyword has its own token
     */
    Symbol(String),

    /**
     * Represent a constant integer value
     */
    Integer(i32),

    /**
     * Represent a "character" token (i.e. value wrapped between single quote)
     * Unlike C like language, in this value allows to have multiple chars,
     * because it uses a word to store this value, so it can store at most 4 chars into it
     * When encounter less than 4 chars, other values will be filled with NULL (0)
     * At least one char is required.
     */
    Char([char; 4]),

    /**
     * Represent a string token (i.e. value wrapped between single quote)
     * The value of this token doesn't include the termination character (0x05)
     */
    String(String),

    /**
     * Represent an operator, can be unary, or binary.
     * The value is the operator string. A string is used because some operators can have multiple
     * characters, like increment "++"
     */
    Operator(String),

    /**
     * Represent the end of a statement (character ;)
     */
    EndOfStatement,

    /**
     * The opening curly brace {
     */
    OpeningBrace,

    /**
     * The closing curly brace }
     */
    ClosingBrace,

    /**
     * The opening parenthesis (
     */
    OpeningParenthesis,

    /**
     * The closing parenthesis )
     */
    ClosingParenthesis,

    /**
     * The opening bracket [
     */
    OpeningBracket,

    /**
     * The closing bracket ]
     */
    ClosingBracket,

    /**
     * The auto keyword. Used to declare variable.
     */
    Auto,

    /**
     * The extrn keyword. Use to declare an external symbol (i.e. symbol not yet found)
     */
    Extern,

    /**
     * The case statement. Use as a branch of a switch statement.
     */
    Case,

    /**
     * The if conditional statement.
     */
    If,

    /**
     * The else statement.
     */
    Else,

    /**
     * The while statement.
     */
    While,

    /**
     * The switch statement.
     */
    Switch,

    /**
     * The goto statement.
     */
    Goto,

    /**
     * The return statement.
     */
    Return,
}

enum TokenizerState {
    Empty,
    AmbiguousOperator,
    SymbolOrKeyword,
    Comment,
    Char,
    String,
    Numeric,
}

struct Tokenizer {
    tokens: Vec<Token>,
    buffer: String,
    state: TokenizerState,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            tokens: Vec::new(),
            buffer: String::new(),
            state: TokenizerState::Empty,
        }
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        self.reset_state();
        self.tokens.clone()
    }

    pub fn push(&mut self, c: char) {
        match self.state {
            TokenizerState::Empty => {
                match c {
                    c if c.is_whitespace() => {}
                    c if c.is_numeric() => {
                        self.state = TokenizerState::Numeric;
                        self.buffer.push(c);
                    }
                    '"' => {
                        self.state = TokenizerState::String;
                    }
                    '\'' => {
                        self.state = TokenizerState::Char;
                    }
                    '(' => self.tokens.push(Token::OpeningParenthesis),
                    ')' => self.tokens.push(Token::ClosingParenthesis),
                    '[' => self.tokens.push(Token::OpeningBracket),
                    ']' => self.tokens.push(Token::ClosingBracket),
                    '{' => self.tokens.push(Token::OpeningBrace),
                    '}' => self.tokens.push(Token::ClosingBrace),
                    ';' => self.tokens.push(Token::EndOfStatement),
                    ',' => self.tokens.push(Token::Operator(String::from(","))),
                    ':' => self.tokens.push(Token::Operator(String::from(":"))),
                    '|' => self.tokens.push(Token::Operator(String::from("|"))),
                    '&' => self.tokens.push(Token::Operator(String::from("&"))),
                    '%' => self.tokens.push(Token::Operator(String::from("%"))),
                    '*' => self.tokens.push(Token::Operator(String::from("*"))),
                    '?' => self.tokens.push(Token::Operator(String::from("?"))),
                    c if ['+', '-', '/', '!', '=', '>', '<'].contains(&c) => {
                        self.state = TokenizerState::AmbiguousOperator;
                        self.buffer.push(c);
                    }
                    _ => {
                        self.state = TokenizerState::SymbolOrKeyword;
                        self.buffer.push(c);
                    }
                }
            }
            TokenizerState::AmbiguousOperator => {
                match c {
                    c if c.is_whitespace() => self.save_token(),
                    c if c.is_numeric() => {
                        self.save_token();
                        self.state = TokenizerState::Numeric;
                        self.buffer.push(c);
                    }

                    // "double" operators
                    c if ['+', '-', '>', '<', '='].contains(&c) && self.buffer.ends_with(c) => {
                        self.buffer.push(c);
                        self.save_token();
                    }

                    // comment "/*"
                    '*' if self.buffer.ends_with('/') => {
                        self.state = TokenizerState::Comment;
                        self.buffer.clear();
                    }

                    // inequalities
                    '=' if ['!', '>', '<'].contains(&self.buffer.chars().next().unwrap()) => { // @todo do not use unwrap
                        self.buffer.push(c);
                        self.save_token();
                    }

                    '"' => {
                        self.save_token();
                        self.state = TokenizerState::String;
                    }
                    '\'' => {
                        self.save_token();
                        self.state = TokenizerState::Char;
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
                        self.save_token();
                        self.push(c);
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
                        self.save_token();
                        self.push(c); // push from initial state
                    },

                    _ => {
                        self.buffer.push(c);
                    }
                }
            }
            TokenizerState::Comment => {
                if c == '/' && self.buffer.pop().is_some_and(|o| o == '*') {
                    self.reset_state();
                    return;
                }

                if c == '*' {
                    self.buffer.push(c);
                }
            }
            TokenizerState::Char => {
                if c == '\'' {
                    self.save_token();
                    return;
                }

                // @todo parse meta char

                self.buffer.push(c);
            }
            TokenizerState::String => {
                if c == '"' {
                    self.save_token();
                    return;
                }

                // @todo parse meta char

                self.buffer.push(c);
            }
            TokenizerState::Numeric => {
                if c.is_numeric() {
                    self.buffer.push(c);
                    return;
                }

                self.save_token();
                self.push(c);
            }
        }
    }

    fn save_and_push(&mut self, token: Token) {
        self.save_token();
        self.tokens.push(token);
    }

    fn save_token(&mut self) {
        match self.state {
            TokenizerState::Empty => {}
            TokenizerState::AmbiguousOperator => self.tokens.push(Token::Operator(self.buffer.clone())),
            TokenizerState::SymbolOrKeyword => {
                // @todo discriminate keywords
                self.tokens.push(Token::Symbol(self.buffer.clone()));
            }
            TokenizerState::Comment => {}
            TokenizerState::Char => {
                let mut chars: [char; 4] = ['\0'; 4];

                if self.buffer.len() < 1 || self.buffer.len() > 4 {
                    println!("Invalid character size") // @todo error handling
                }

                self.buffer.chars().enumerate().for_each(|(i, c)| chars[i] = c);
                self.tokens.push(Char(chars));
            }
            TokenizerState::String => self.tokens.push(Token::String(self.buffer.clone())),
            TokenizerState::Numeric => {
                match self.buffer.parse() {
                    Ok(i) => self.tokens.push(Token::Integer(i)),
                    Err(_) => println!("Error during parsing number token") // @todo error handling
                }
            }
        }

        self.reset_state();
    }

    fn reset_state(&mut self) {
        self.state = TokenizerState::Empty;
        self.buffer.clear();
    }
}

fn tokenize_file(file: &Path) -> Vec<Token> {
    let mut file = File::open(file).expect("Unable to read file"); // @todo match error
    let mut buf: [u8; 256] = [0; 256];

    let mut tokenizer = Tokenizer::new();

    loop {
        match file.read(buf.as_mut()) {
            Ok(size) => {
                for i in 0..size {
                    tokenizer.push(char::from(buf[i]));
                }

                if size < 256 {
                    break
                }
            }
            Err(_) => {
                println!("ERROR");

                break
            }
        }
    }

    for token in tokenizer.tokens() {
        println!("{:?}", token);
    }

    return tokenizer.tokens();

    //println!("File content {:?}", tokenizer.tokens())
}

fn main() {
    tokenize_file(Path::new("../example/printn.b"));
}
