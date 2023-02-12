use std::fmt;

/// This is the root node of our AST
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(String),
    Return, // TODO: add expression
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(ident) => write!(f, "let {};", ident),
            Statement::Return => write!(f, "return;"),
        }
    }
}
