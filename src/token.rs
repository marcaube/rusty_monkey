#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Special types
    Illegal, // for unsupported tokens
    Eof,     // mark the end of file

    // Identifiers + literals
    Ident, // add, foobar, x, y, ...
    Int,   // 1343456

    // Operators
    Assign, // =
    Plus,   // +

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
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}