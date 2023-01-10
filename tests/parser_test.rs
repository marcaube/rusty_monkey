use rusty_monkey::{ast::Statement, lexer::Lexer, parser::Parser};

#[test]
fn test_let_statement() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        3,
        "Expected 3 program statements, got {}",
        program.statements.len()
    );

    assert_eq!(
        program.statements,
        vec![
            Statement::Let("x".to_string()),
            Statement::Let("y".to_string()),
            Statement::Let("foobar".to_string()),
        ],
        "Wrong program statements"
    );
}

#[test]
fn test_return_statement() {
    let input = "
        return 5;
        return 10;
        return 993322;
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        3,
        "Expected 3 program statements, got {}",
        program.statements.len()
    );

    assert_eq!(
        program.statements,
        vec![
            Statement::Return,
            Statement::Return,
            Statement::Return,
        ],
        "Wrong program statements"
    );

}

fn check_parser_errors(parser: &Parser) {
    assert_eq!(parser.errors.len(), 0, "Parser errors: {:?}", parser.errors,);
}
