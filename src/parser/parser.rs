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
            _ => {
                self.cursor = self.cursor - 1; // Rollback to previous token
                println!("expression {:?}", self.parse_rvalue());
                self.check(Token::EndOfStatement);
            }
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

    fn parse_rvalue(&mut self) -> String {
        return self.parse_assign_left().join(" ") + self.parse_or_expr().as_str();
    }

    fn parse_or_expr(&mut self) -> String {
        let mut expr = self.parse_and_expr();

        while self.current().is_operator_str("||") {
            self.next();

            expr += ", ";
            expr += self.parse_and_expr().as_str();
        }

        return format!("and({})", expr);
    }

    fn parse_and_expr(&mut self) -> String {
        let mut expr = self.parse_equality_expr();

        while self.current().is_operator_str("&&") {
            self.next();

            expr += ", ";
            expr += self.parse_equality_expr().as_str();
        }

        return format!("or({})", expr);
    }

    fn parse_equality_expr(&mut self) -> String {
        let mut expr = self.parse_shift_expr();

        loop {
            match self.current() {
                Token::Operator(op) if op == "==" => {
                    self.next();
                    expr = format!("equals({}, {})", expr, self.parse_shift_expr());
                },
                Token::Operator(op) if op == "!=" => {
                    self.next();
                    expr = format!("different({}, {})", expr, self.parse_shift_expr());
                },
                _ => break
            }
        }

        return expr;
    }

    fn parse_shift_expr(&mut self) -> String {
        let mut expr = self.parse_add_expr();

        loop {
            match self.current() {
                Token::Operator(op) if op == ">>" => {
                    self.next();
                    expr = format!("shift_right({}, {})", expr, self.parse_add_expr());
                },
                Token::Operator(op) if op == "<<" => {
                    self.next();
                    expr = format!("shift_left({}, {})", expr, self.parse_add_expr());
                },
                _ => break
            }
        }

        return expr;
    }

    fn parse_add_expr(&mut self) -> String {
        let mut expr = self.parse_mul_expr();

        loop {
            match self.current() {
                Token::Operator(op) if op == "+" => {
                    self.next();
                    expr = format!("add({}, {})", expr, self.parse_mul_expr());
                },
                Token::Operator(op) if op == "-" => {
                    self.next();
                    expr = format!("minus({}, {})", expr, self.parse_mul_expr());
                },
                _ => break
            }
        }

        return expr;
    }

    fn parse_mul_expr(&mut self) -> String {
        let mut expr = self.parse_prefix_expr();

        loop {
            match self.current() {
                Token::Operator(op) if op == "*" => {
                    self.next();
                    expr = format!("multiply({}, {})", expr, self.parse_prefix_expr());
                },
                Token::Operator(op) if op == "/" => {
                    self.next();
                    expr = format!("divide({}, {})", expr, self.parse_prefix_expr());
                },
                Token::Operator(op) if op == "%" => {
                    self.next();
                    expr = format!("mod({}, {})", expr, self.parse_prefix_expr());
                },
                _ => break
            }
        }

        return expr;
    }

    fn parse_assign_left(&mut self) -> Vec<String> {
        let mut ret = Vec::new();

        loop {
            let cursor = self.cursor;
            let left = self.parse_prefix_expr();

            if !self.current().is_operator('=') {
                self.cursor = cursor; // rollback to position before parsing
                break;
            }

            match self.next() {
                Token::Operator(op) if (op == "|" || op == "&" || op == "==" || op == "!=" || op == "<" || op == "<=" || op == ">" || op == ">=" || op == ">>" || op == "<<" || op == "-" || op == "+" || op == "%" || op == "*" || op == "/") => {
                    ret.push(format!("{} {}= ", left, op))
                },
                _ => {
                    ret.push(format!("{} = ", left))
                }
            }
        }

        return ret;
    }

    fn parse_prefix_expr(&mut self) -> String {
        let mut expr = String::new();

        loop {
            match self.current() {
                Token::Operator(op) if (op == "!" || op == "-" || op == "&" || op == "*" || op == "++" || op == "--") => {
                    self.next();
                    expr += format!("unary({}) ", op).as_str();
                },
                _ => break
            }
        }

        expr += self.parse_postfix_expr().as_str();

        return expr;
    }

    fn parse_postfix_expr(&mut self) -> String {
        let mut expr = self.parse_atomic_expr();

        loop {
            match self.current() {
                Token::OpeningBracket => todo!("implements array access"),
                Token::OpeningParenthesis => {
                    expr = format!("call({}, [{}])", expr, self.parse_function_call_arguments().join(", "));
                },
                Token::Operator(op) if (op == "++" || op == "--") => {
                    self.next();

                    expr = format!("postfix({}, {})", expr, op);
                },
                _ => break,
            }
        }

        return expr;
    }

    fn parse_function_call_arguments(&mut self) -> Vec<String> {
        self.check(Token::OpeningParenthesis);

        let mut args = Vec::new();

        while self.current() != Token::ClosingParenthesis {
            args.push(self.parse_rvalue());

            match self.current() {
                Token::Operator(op) if op == "," => {
                    self.next();
                },
                Token::ClosingParenthesis => {},
                tok => panic!("unexpected token {:?} expecting , or )", tok)
            }
        }

        self.next();

        return args;
    }

    fn parse_atomic_expr(&mut self) -> String {
        match self.next() {
            Token::Integer(ival) => format!("int({})", ival),
            Token::String(str) => format!("string({})", str),
            Token::Char(chars) => format!("char({})", chars[0]),
            Token::Symbol(var) => format!("var({})", var),
            Token::OpeningParenthesis => {
                let expr = format!("({})", self.parse_rvalue());

                self.check(Token::ClosingParenthesis);

                return expr;
            },
            tok => panic!("expr: Invalid token {:?} expecting int, string, char, symbol or (", tok),
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

    fn check(&mut self, expecting: Token) {
        let token = self.next();

        if token != expecting {
            panic!("invalid token {:?} expecting {:?}", token, expecting);
        }
    }
}
