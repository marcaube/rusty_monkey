use std::fmt;

/// This is the root node of our AST
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(String),
    Return, // TODO: add expression
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(ident) => write!(f, "let {};", ident),
            Statement::Return => write!(f, "return;"),
            _ => write!(f, "Unsupported!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(String),
    Integer(usize),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::Integer(int) => write!(f, "{}", int),
        }
    }
}
