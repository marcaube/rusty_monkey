use crate::token::{Token, TokenType};

pub struct Lexer {
    pub input: String,        // the source code
    pub position: usize,      // current position in input (points to current char)
    pub read_position: usize, // current reading position in input (after current char)
    pub ch: char,             // current char under examination
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\u{0}',
        };
        lexer.read_char();
        lexer
    }

    /// Yield the next character and advance our position
    pub fn read_char(self: &mut Self) {
        self.ch = self
            .input
            .chars()
            .nth(self.read_position)
            .unwrap_or('\u{0}');

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Look at the current character and return the appropriate Token
    pub fn next_token(self: &mut Self) -> Token {
        let tok: Token;

        match self.ch {
            '=' => tok = new_token(TokenType::Assign, self.ch),
            '+' => tok = new_token(TokenType::Plus, self.ch),
            '(' => tok = new_token(TokenType::Lparen, self.ch),
            ')' => tok = new_token(TokenType::Rparen, self.ch),
            '{' => tok = new_token(TokenType::Lbrace, self.ch),
            '}' => tok = new_token(TokenType::Rbrace, self.ch),
            ',' => tok = new_token(TokenType::Comma, self.ch),
            ';' => tok = new_token(TokenType::Semicolon, self.ch),
            _ => {
                tok = Token {
                    token_type: TokenType::Eof,
                    literal: "".to_string(),
                };
            }
        }

        self.read_char();
        tok
    }
}

/// Helper method to create a new Token from a type and a char
fn new_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}
