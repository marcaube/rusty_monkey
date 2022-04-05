use rusty_monkey::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[test]
fn test_next_token() {
    let input = "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y
        };

        let result = add(five, ten);
    ";

    let tests = [
        (TokenType::Let, "let"),
        (TokenType::Ident, "five"),
        (TokenType::Assign, "="),
        (TokenType::Int, "5"),
        // (TokenType::Semicolon, ";"),
        // (TokenType::Let, "let"),
        // (TokenType::Ident, "ten"),
        // (TokenType::Assign, "="),
        // (TokenType::Int, "10"),
        // (TokenType::Semicolon, ";"),
        // (TokenType::Let, "let"),
        // (TokenType::Ident, "add"),
        // (TokenType::Assign, "="),
        // (TokenType::Function, "fn"),
        // (TokenType::Lparen, "("),
        // (TokenType::Ident, "x"),
        // (TokenType::Comma, ","),
        // (TokenType::Ident, "y"),
        // (TokenType::Rparen, ")"),
        // (TokenType::Lbrace, "{"),
        // (TokenType::Ident, "x"),
        // (TokenType::Plus, "+"),
        // (TokenType::Ident, "y"),
        // (TokenType::Semicolon, ";"),
        // (TokenType::Rbrace, "}"),
        // (TokenType::Semicolon, ";"),
        // (TokenType::Let, "let"),
        // (TokenType::Ident, "result"),
        // (TokenType::Assign, "="),
        // (TokenType::Ident, "add"),
        // (TokenType::Lparen, "("),
        // (TokenType::Ident, "five"),
        // (TokenType::Comma, ","),
        // (TokenType::Ident, "ten"),
        // (TokenType::Rparen, ")"),
        // (TokenType::Semicolon, ";"),
        // (TokenType::Eof, ""),
    ];

    let mut lexer = Lexer::new(input);

    for (i, (token_type, literal)) in tests.iter().enumerate() {
        let tok: Token = lexer.next_token();

        assert_eq!(
            &tok.token_type, token_type,
            "tests[{}] - token_type wrong. expected={:?}, got={:?}",
            i, token_type, tok.token_type
        );
        assert_eq!(
            &tok.literal, literal,
            "tests[{}] - literal wrong. expected={:?}, got={:?}",
            i, literal, tok.literal
        );
    }
}
