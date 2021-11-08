use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: i32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.literal {
            Some(str) => write!(
                f,
                "{} {} {}",
                self.token_type,
                self.lexeme,
                str,
            ),
            None => write!(
                f,
                "{} {}",
                self.token_type,
                self.lexeme
            )
        }
        
    }
}
