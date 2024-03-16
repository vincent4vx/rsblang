use std::thread::current;
use crate::parser::structure::Program;
use crate::tokenizer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            cursor: 0
        }
    }

    pub fn parse(tokens: Vec<Token>) {
        Self::new(tokens).parse_program();
    }

    fn parse_program(&mut self) {
        match self.next() {
            Token::Symbol(name) => {
                match self.current() {
                    Token::OpeningParenthesis => self.parse_function(name),
                    _ => self.parse_global(name),
                }
            },
            _ => panic!("Expecting symbol")
        }
    }

    fn parse_function(&mut self, name: String) {
        match self.next() {
            Token::OpeningParenthesis => {},
            _ => panic!("Expecting (")
        }

        let mut arguments: Vec<String> = Vec::new();

        loop {
            match &self.next() {
                Token::ClosingParenthesis => break,
                Token::Symbol(name) => {
                    arguments.push(String::from(name));

                    match &self.next() {
                        Token::ClosingParenthesis => break,
                        token if token.is_operator(',') => continue,
                        _ => panic!("Invalid token")
                    }
                },
                _ => panic!("Invalid token")
            }
        }

        self.parse_statement();
    }

    fn parse_statement(&mut self) {
        match self.next() {
            Token::Auto => self.parse_auto(),
            Token::Extern => self.parse_extern(),
            Token::Symbol(_) if self.current().is_operator(':') => println!("label"),
            Token::Case => println!("case"),
            Token::OpeningBrace => self.parse_statements(),
            Token::If => self.parse_if(),
            Token::While => println!("while"),
            Token::Switch => println!("switch"),
            Token::Goto => println!("goto"),
            Token::Return => println!("return"),
            token => println!("expression {:?}", token)
        }
    }

    fn parse_statements(&mut self) {
        println!("statements");

        loop {
            match self.current() {
                Token::ClosingBrace => {
                    self.next();
                    break
                },
                _ => self.parse_statement(),
            }
        }
    }

    fn parse_auto(&mut self) {
        match self.next() {
            Token::Symbol(name) => {
                println!("var {}", name);

                match self.next() {
                    Token::Integer(value) => {
                        println!("var {} = {}", name, value);
                        match self.next() {
                            Token::Operator(op) if op == "," => self.parse_auto(),
                            Token::EndOfStatement => return,
                            t => panic!("invalid token {:?}", t)
                        }
                    },
                    Token::Operator(op) if op == "," => self.parse_auto(),
                    Token::EndOfStatement => return,
                    t => panic!("invalid token {:?}", t)
                }
            },
            token => panic!("expecting symbol {:?}", token)
        }
    }

    fn parse_extern(&mut self) {
        loop {
            match self.next() {
                Token::Symbol(name) => println!("extern {}", name),
                Token::Operator(op) if op == "," => {},
                Token::EndOfStatement => return,
                token => panic!("invalid token {:?}", token)
            }
        }
    }

    fn parse_if(&mut self) {
        match self.next() {
            Token::OpeningParenthesis => {},
            token => panic!("invalid token {:?}", token)
        }

        println!("if (");

        println!("cond: {:?}", self.parse_rvalue());

        match self.next() {
            Token::ClosingParenthesis => {},
            token => panic!("if: invalid token {:?} expecting )", token)
        }

        println!(") then");

        self.parse_statement();

        println!("endif")
    }

    fn parse_rvalue(&mut self) -> Option<String> {
        let current = self.current();

        match current {
            Token::OpeningParenthesis => {
                self.next();

                let expr = self.parse_rvalue();

                match self.next() {
                    Token::ClosingParenthesis => expr,
                    token => panic!("invalid token {:?}", token)
                }
            },

            Token::Integer(ival) => Some(format!("int({})", ival)),
            Token::Char(chars) => Some(format!("char({})", chars[0])),
            Token::String(str) => Some(format!("string({})", str)),

            Token::Operator(op) if (op == "-" || op == "!") => {
                self.next();

                match self.parse_rvalue() {
                    Some(v) => Some(format!("unary({} {})", op, v)),
                    None => panic!("invalid unary")
                }
            },

            Token::Operator(op) if (op == "&" || op == "++" || op == "--") => {
                self.next();

                match self.parse_lvalue() {
                    Some(v) => Some(format!("unary({} {})", op, v)),
                    None => panic!("invalid unary")
                }
            },
            _ => {
                match self.parse_lvalue() {
                    Some(v) => {
                        match self.current() {
                            Token::Operator(op) if op == "=" => self.parse_assign(v),
                            Token::Operator(op) if (op == "++" || op == "--") => Some(format!("unary({} {})", v, op)),
                            tok => panic!("unexpected token ({:?})", tok)
                        }
                    },
                    None => panic!("invalid lvalue")
                }
            },
        }
    }

    fn parse_assign(&mut self, left: String) -> Option<String> {
        if !self.next().is_operator('=') {
            panic!("assign: invalid token, expect =");
        }

        match self.current() {
            Token::Operator(op) if (op == "|" || op == "&" || op == "==" || op == "!=" || op == "<" || op == "<=" || op == ">" || op == ">=" || op == ">>" || op == "<<" || op == "-" || op == "+" || op == "%" || op == "*" || op == "/") => {
                match self.parse_rvalue() {
                    Some(right) => Some(format!("assign+op({} {}= {})", left, op, right)),
                    None => panic!("assign: invalid right rvalue")
                }
            },
            _ => {
                match self.parse_rvalue() {
                    Some(right) => Some(format!("assign({} = {})", left, right)),
                    None => panic!("assign: invalid right rvalue")
                }
            }
        }
    }

    fn parse_lvalue(&mut self) -> Option<String> {
        match self.next() {
            Token::Symbol(var) => Some(format!("var({})", var)),
            Token::Operator(op) if (op == "*") => {
                match self.parse_rvalue() {
                    Some(v) => Some(format!("deref({})", v)),
                    None => panic!("deref: invalid rvalue")
                }
            },
            _ => {
                match self.parse_rvalue() {
                    Some(v) => {
                        if self.next() != Token::OpeningBracket {
                            panic!("arr: expecting [, go {:?}", self.current());
                        }

                        match self.parse_rvalue() {
                            Some(index) => {
                                if self.next() != Token::ClosingBracket {
                                    panic!("arr: expecting ], go {:?}", self.current());
                                }

                                Some(format!("var({}[{}])", v, index))
                            },
                            None => panic!("arr: invalid rvalue index")
                        }
                    },
                    None => panic!("arr: invalid rvalue")
                }
            }
        }
    }

    fn parse_global(&mut self, name: String) {
        todo!()
    }

    fn next(&mut self) -> Token {
        let token = self.tokens[self.cursor].clone(); // @todo copy instead of clone
        self.cursor += 1;

        token
    }

    fn current(&self) -> Token {
        self.tokens[self.cursor].clone()
    }
}
