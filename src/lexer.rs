use crate::token::{lookup_ident, Token};

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

    /// Similar to read_char() but does not increment positions
    pub fn peek_char(&mut self) -> char {
        self.input.chars().nth(self.read_position).unwrap_or('\0')
    }

    /// Look at the current character and return the appropriate Token
    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        self.eat_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token::Eq;
                } else {
                    tok = Token::Assign;
                }
            }
            '+' => tok = Token::Plus,
            '-' => tok = Token::Minus,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token::Noteq;
                } else {
                    tok = Token::Bang;
                }
            }
            '/' => tok = Token::Slash,
            '*' => tok = Token::Asterisk,
            '<' => tok = Token::Lt,
            '>' => tok = Token::Gt,
            '(' => tok = Token::Lparen,
            ')' => tok = Token::Rparen,
            '{' => tok = Token::Lbrace,
            '}' => tok = Token::Rbrace,
            ',' => tok = Token::Comma,
            ';' => tok = Token::Semicolon,
            '\0' => {
                // Early return to prevent advancing the read position
                return Token::Eof;
            }
            _ => {
                if is_letter(self.ch) {
                    let identifier = self.read_identifier();

                    return lookup_ident(identifier);
                } else if is_digit(self.ch) {
                    let literal = self.read_number();

                    return Token::Int(literal.to_string());
                } else {
                    tok = Token::Illegal;
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

    fn read_number(&mut self) -> &str {
        // TODO: add support for floats, hex, octal and binary
        let start = self.position;

        while is_digit(self.ch) {
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

/// Helper method to check if a char is a letter
/// This function has a large impact on the language our interpreter will support
fn is_letter(ch: char) -> bool {
    ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ('0'..='9').contains(&ch)
}
