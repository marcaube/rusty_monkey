use crate::token::{lookup_ident, Token, TokenType};

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
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    /// Yield the next character and advance our position
    pub fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0');

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Look at the current character and return the appropriate Token
    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        self.eat_whitespace();

        match self.ch {
            '=' => tok = new_token(TokenType::Assign, self.ch),
            '+' => tok = new_token(TokenType::Plus, self.ch),
            '(' => tok = new_token(TokenType::Lparen, self.ch),
            ')' => tok = new_token(TokenType::Rparen, self.ch),
            '{' => tok = new_token(TokenType::Lbrace, self.ch),
            '}' => tok = new_token(TokenType::Rbrace, self.ch),
            ',' => tok = new_token(TokenType::Comma, self.ch),
            ';' => tok = new_token(TokenType::Semicolon, self.ch),
            '\0' => tok = new_token(TokenType::Eof, '\0'),
            _ => {
                if is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    let token_type = lookup_ident(identifier);

                    // Early return to prevent advancing the read position
                    return Token {
                        token_type,
                        literal: identifier.to_string(),
                    };
                } else {
                    tok = new_token(TokenType::Illegal, self.ch)
                }
            }
        }

        self.read_char();
        tok
    }

    /// Read a multi-char Token, like identifiers
    fn read_identifier(&mut self) -> &str {
        let start = self.position;

        while is_letter(self.ch) {
            self.read_char();
        }

        &self.input[start..self.position]
    }

    fn eat_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

/// Helper method to create a new Token from a type and a char
fn new_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}

/// Helper method to check if a char is a letter
/// This function has a large impact on the language our interpreter will support
fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}
