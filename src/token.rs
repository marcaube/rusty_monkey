#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Special types
    Illegal, // for unsupported tokens
    Eof,     // mark the end of file

    // Identifiers + literals
    Ident, // add, foobar, x, y, ...
    Int,   // 1343456

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /
    Lt,       // <
    Gt,       // >
    Eq,       // ==
    Noteq,    // !=

    // Delimiters
    Comma,     // ,
    Semicolon, // ;

    Lparen, // (
    Rparen, // )
    Lbrace, // {
    Rbrace, // }

    // Keywords
    Function, // fn
    Let,      // let
    True,     // true
    False,    // false
    If,       // if
    Else,     // else
    Return,   // return
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    // TODO: extract this mapping to a constant?
    match ident {
        "let" => TokenType::Let,
        "fn" => TokenType::Function,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}
