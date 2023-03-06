use rusty_monkey::{
    ast::{Expression, Program, Statement},
    lexer::Lexer,
    parser::Parser,
};

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

    assert_no_parser_errors(&parser);
    assert_program_length(&program, 3);

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

    assert_no_parser_errors(&parser);
    assert_program_length(&program, 3);

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

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_no_parser_errors(&parser);
    assert_program_length(&program, 1);

    assert_eq!(
        program.statements,
        vec![
            Statement::Expression(Expression::Identifier("foobar".to_string()))
        ],
        "Wrong program statements"
    )
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_no_parser_errors(&parser);
    assert_program_length(&program, 1);

    assert_eq!(
        program.statements,
        vec![
            Statement::Expression(Expression::Integer(5))
        ],
        "Wrong program statements"
    )
}

#[test]
fn test_parsing_prefix_expressions() {
    let test_cases = vec![
        ("!5;", "!", 5),
        ("-15;", "-", 15),
    ];

    for (input, operator, value) in test_cases {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_no_parser_errors(&parser);
        assert_program_length(&program, 1);

        assert_eq!(
            program.statements,
            vec![
                Statement::Expression(
                    Expression::Prefix(
                        operator.to_string(),
                        Box::new(Expression::Integer(value))
                    )
                )
            ],
            "Wrong program statements"
        )
    }
}

#[test]
fn test_parsing_infix_expressions() {
    let test_cases = vec![
        ("5 + 5;", 5, "+", 5),
        ("5 - 5;", 5, "-", 5),
        ("5 * 5;", 5, "*", 5),
        ("5 / 5;", 5, "/", 5),
        ("5 > 5;", 5, ">", 5),
        ("5 < 5;", 5, "<", 5),
        ("5 == 5;", 5, "==", 5),
        ("5 != 5;", 5, "!=", 5),
    ];

    for (input, left_value, operator, right_value) in test_cases {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_no_parser_errors(&parser);
        assert_program_length(&program, 1);

        assert_eq!(
            program.statements,
            vec![
                Statement::Expression(
                    Expression::Infix(
                        Box::new(Expression::Integer(left_value)),
                        operator.to_string(),
                        Box::new(Expression::Integer(right_value)),
                    )
                )
            ],
            "Wrong program statements"
        )
    }
}

fn assert_no_parser_errors(parser: &Parser) {
    assert_eq!(parser.errors.len(), 0, "Parser errors: {:?}", parser.errors,);
}

fn assert_program_length(program: &Program, expected_length: usize) {
    assert_eq!(
        program.statements.len(),
        expected_length,
        "Expected {} program statements, got {}",
        expected_length,
        program.statements.len(),
    )
}
