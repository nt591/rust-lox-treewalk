use crate::{
    token::{self, Token},
    token_type::TokenType,
};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: Vec<char>) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self, tokens: &mut Vec<Token>) -> () {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(tokens);
        }

        tokens.push(Token::new(TokenType::Eof, String::new(), None, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    fn add_token_with_literal(
        &self,
        token_type: TokenType,
        literal: Option<String>,
        tokens: &mut Vec<Token>,
    ) -> () {
        let text: String = self.source[self.start..self.current].iter().collect();
        tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn add_token(&self, token_type: TokenType, tokens: &mut Vec<Token>) -> () {
        self.add_token_with_literal(token_type, None, tokens)
    }

    fn scan_token(&mut self, tokens: &mut Vec<Token>) -> () {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, tokens),
            ')' => self.add_token(TokenType::RightParen, tokens),
            '{' => self.add_token(TokenType::LeftBrace, tokens),
            '}' => self.add_token(TokenType::RightBrace, tokens),
            ',' => self.add_token(TokenType::Comma, tokens),
            '.' => self.add_token(TokenType::Dot, tokens),
            '-' => self.add_token(TokenType::Minus, tokens),
            '+' => self.add_token(TokenType::Plus, tokens),
            ';' => self.add_token(TokenType::Semicolon, tokens),
            '*' => self.add_token(TokenType::Star, tokens),
            _ => (),
        }
    }
}
