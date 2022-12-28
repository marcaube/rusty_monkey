#[derive(Debug, PartialEq)]
pub enum Token {
    // Special types
    Illegal, // for unsupported tokens
    Eof,     // mark the end of file

    // Identifiers + literals
    Ident(String), // add, foobar, x, y, ...
    Int(String),   // 1343456

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

    Comment,  // //
}

pub fn lookup_ident(ident: &str) -> Token {
    // TODO: extract this mapping to a constant?
    match ident {
        "let" => Token::Let,
        "fn" => Token::Function,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident.to_string()),
    }
}
