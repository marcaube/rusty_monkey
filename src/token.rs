use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
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
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Special types
            Token::Illegal => write!(f, "ILLEGAL"),
            Token::Eof => write!(f, "EOF"),

            // Identifiers + literals
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Int(int) => write!(f, "{}", int),

            // Operators
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Eq => write!(f, "=="),
            Token::Noteq => write!(f, "!="),

            // Delimiters
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ","),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbrace => write!(f, "["),
            Token::Rbrace => write!(f, "]"),

            // Keywords
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
        }
    }
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
