use rusty_monkey::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[test]
fn test_next_token() {
    let input = "=+(){},;";

    let tests = [
        (TokenType::Assign, "="),
        (TokenType::Plus, "+"),
        (TokenType::Lparen, "("),
        (TokenType::Rparen, ")"),
        (TokenType::Lbrace, "{"),
        (TokenType::Rbrace, "}"),
        (TokenType::Comma, ","),
        (TokenType::Semicolon, ";"),
        (TokenType::Eof, ""),
    ];

    let mut lexer = Lexer::new(input);

    for (i, (token_type, literal)) in tests.iter().enumerate() {
        let tok: Token = lexer.next_token();

        assert_eq!(
            &tok.token_type, token_type,
            "tests[{}] - token_type wrong.",
            i
        );
        assert_eq!(&tok.literal, literal, "tests[{}] - literal wrong.", i);
    }
}
