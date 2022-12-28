use rusty_monkey::{
    lexer::Lexer,
    token::Token,
};

#[test]
fn test_next_token() {
    let input = "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y
        };

        // One line comment

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        // Multi-line
        // comment
        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        // Comment at the end";

    let tests = [
        Token::Let,
        Token::Ident("five".to_string()),
        Token::Assign,
        Token::Int("5".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("ten".to_string()),
        Token::Assign,
        Token::Int("10".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".to_string()),
        Token::Assign,
        Token::Function,
        Token::Lparen,
        Token::Ident("x".to_string()),
        Token::Comma,
        Token::Ident("y".to_string()),
        Token::Rparen,
        Token::Lbrace,
        Token::Ident("x".to_string()),
        Token::Plus,
        Token::Ident("y".to_string()),
        Token::Rbrace,
        Token::Semicolon,
        Token::Comment,
        Token::Let,
        Token::Ident("result".to_string()),
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::Lparen,
        Token::Ident("five".to_string()),
        Token::Comma,
        Token::Ident("ten".to_string()),
        Token::Rparen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int("5".to_string()),
        Token::Semicolon,
        Token::Int("5".to_string()),
        Token::Lt,
        Token::Int("10".to_string()),
        Token::Gt,
        Token::Int("5".to_string()),
        Token::Semicolon,
        Token::Comment,
        Token::Comment,
        Token::If,
        Token::Lparen,
        Token::Int("5".to_string()),
        Token::Lt,
        Token::Int("10".to_string()),
        Token::Rparen,
        Token::Lbrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::Rbrace,
        Token::Else,
        Token::Lbrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::Rbrace,
        Token::Int("10".to_string()),
        Token::Eq,
        Token::Int("10".to_string()),
        Token::Semicolon,
        Token::Int("10".to_string()),
        Token::Noteq,
        Token::Int("9".to_string()),
        Token::Semicolon,
        Token::Comment,
        Token::Eof,
    ];

    let mut lexer = Lexer::new(input);

    for (i, token_type) in tests.iter().enumerate() {
        let tok: Token = lexer.next_token();

        assert_eq!(
            &tok, token_type,
            "tests[{}] - token_type wrong. expected={:?}, got={:?}",
            i, token_type, tok
        );
    }
}
