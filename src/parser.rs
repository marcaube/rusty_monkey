use crate::{
    ast::{Expression, Program, Statement},
    lexer::Lexer,
    token::Token,
};

// Operator precedence, or "order of operations"
enum Precedence {
    Lowest,
    // Equals,      // ==
    // LessGreater, // < or >
    // Sum,         // +
    // Product,     // *
    // Prefix,      // -X or !X
    // Call,        // my_function(X)
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

/// Basic implementation of a recursive descent parser
/// with top down operator precedence (Pratt parser).
impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            // Similar to the lexer's `position` and `read_position`, but with tokens
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: vec![],
        };

        // Read two tokens, so current_token and peek_token are both set
        parser.next_token();
        parser.next_token();

        parser
    }

    /// Advance both reading cursors (current_token and peek_token)
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone(); // Token is not Copy, because Strings in identifiers are not
        self.peek_token = self.lexer.next_token();
    }

    /// Parse all the statements and expressions in the program
    pub fn parse_program(&mut self) -> Program {
        let mut statements = vec![];

        while !self.current_token_is(Token::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }

            self.next_token();
        }

        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    /// Parse a statement in the form of `let x = <expression>`
    fn parse_let_statement(&mut self) -> Option<Statement> {
        // Advance to skip the `let` token
        self.next_token();

        // Parse the variable name
        // TODO: find a way to use expect_peek_token here as well
        let identifier = match self.current_token.clone() {
            Token::Ident(name) => name,
            _ => {
                self.errors.push(format!(
                    "Expected next token to be Ident, got {:?} instead",
                    self.current_token
                ));

                return None;
            }
        };

        // Look for and consume the equal sign
        if !self.expect_peek_token(Token::Assign) {
            return None;
        }

        // Skip the tokens until we encounter a semicolon
        // TODO: parse the right-hand side of the expression
        while !self.current_token_is(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let(identifier))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        // Advance to skip the `return` token
        self.next_token();

        // Skip the tokens until we encounter a semicolon
        // TODO: parse the return expression
        while !self.current_token_is(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Return)
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        // TODO: this seems a bit convoluted just to get at the value...
        match expression {
            Some(exp) => Some(Statement::Expression(exp)),
            _ => None,
        }
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Expression> {
        self.parse_prefix(self.current_token.clone())
    }

    // Parse function for tokens in the prefix position
    fn parse_prefix(&mut self, token: Token) -> Option<Expression> {
        match token {
            Token::Ident(ident) => Some(Expression::Identifier(ident)),
            _ => None,
        }
    }

    fn current_token_is(&mut self, expected_token: Token) -> bool {
        self.current_token == expected_token
    }

    fn peek_token_is(&mut self, expected_token: Token) -> bool {
        self.peek_token == expected_token
    }

    /// Check whether the next token is of the expected type
    /// and if true, advance to the next token
    fn expect_peek_token(&mut self, expected_token: Token) -> bool {
        if self.peek_token_is(expected_token.clone()) {
            self.next_token();

            return true;
        }

        self.peek_error(expected_token);

        false
    }

    fn peek_error(&mut self, token_type: Token) {
        self.errors.push(format!(
            "Expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token
        ));
    }
}
