#![allow(unused)]
use once_cell::sync::Lazy;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::AND);
    m.insert("class", TokenType::CLASS);
    m.insert("else", TokenType::ELSE);
    m.insert("false", TokenType::FALSE);
    m.insert("for", TokenType::FOR);
    m.insert("fun", TokenType::FUN);
    m.insert("if", TokenType::IF);
    m.insert("nil", TokenType::NIL);
    m.insert("or", TokenType::OR);
    m.insert("print", TokenType::PRINT);
    m.insert("return", TokenType::RETURN);
    m.insert("super", TokenType::SUPER);
    m.insert("this", TokenType::THIS);
    m.insert("true", TokenType::TRUE);
    m.insert("var", TokenType::VAR);
    m.insert("while", TokenType::WHILE);
    m
});

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u64,
    literal: Option<Box<dyn Any>>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({}, line {})", self.lexeme, self.line)
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        println!("len: {:?}", source.len());
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn error(&self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&self, line: usize, location: String, message: String) {
        panic!("[line {}] Error {}:  {}", line, location, message);
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        if let Some(c) = self.source.chars().nth(self.current - 1) {
            c
        } else {
            panic!("gg lol");
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        let lexeme: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens.push(Token {
            token_type,
            lexeme,
            line: self.line as u64,
            literal,
        });
        println!("Token: {:?}", self.tokens.last());
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            line: self.line as u64,
            literal: None,
        });
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            '!' => self.add_token_if('=', TokenType::BANG_EQUAL, TokenType::BANG),
            '=' => self.add_token_if('=', TokenType::EQUAL_EQUAL, TokenType::EQUAL),
            '<' => self.add_token_if('=', TokenType::LESS_EQUAL, TokenType::LESS),
            '>' => self.add_token_if('=', TokenType::GREATER_EQUAL, TokenType::GREATER),
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_next('*') {
                    self.c_style_comment();
                } else {
                    self.add_token(TokenType::SLASH, None)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number()
                } else if (c.is_alphabetic()) {
                    self.identifier()
                } else {
                    self.error(self.line, "Unexpected character.".to_string())
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphabetic() {
            self.advance();
        }

        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        if let Some(token_type) = KEYWORDS.get(text.as_str()) {
            self.add_token(token_type.clone(), None);
        } else {
            self.add_token(TokenType::IDENTIFIER, None);
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.".to_string());
        }

        self.advance();

        let value: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 1)
            .collect();
        self.add_token(TokenType::STRING, Some(Box::new(value)));
    }

    fn c_style_comment(&mut self) {
        while (self.peek() != '*' || self.peek_next() != '/') && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            self.error(self.line, "Unclosed C style comment.".to_string());
        }
        self.advance();
        self.advance();
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let num: f64 = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>()
            .parse::<f64>()
            .unwrap();

        self.add_token(TokenType::NUMBER, Some(Box::new(num)));
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn add_token_if(&mut self, c: char, yes: TokenType, no: TokenType) {
        if self.match_next(c) {
            self.add_token(yes, None)
        } else {
            self.add_token(no, None)
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }
}

/*
 * leading decimal point support
 *
*/
