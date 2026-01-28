use crate::{expr::Expr, scanner::Token};

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
}

impl Stmt {
    /// Accept a visitor.
    ///
    /// # Errors
    ///
    /// Returns an error if the visitor encounters an error.
    pub fn accept<Visitor>(&self, visitor: &mut Visitor) -> Result<(), Visitor::Error>
    where
        Visitor: StmtVisitor<()>,
    {
        match self {
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Expression(expr) => visitor.visit_expression(expr),
            Stmt::Var(name, expr) => visitor.visit_variable_declaration(name, expr),
        }
    }
}

pub trait StmtVisitor<R> {
    /// The error type returned by the visitor.
    type Error;

    /// Visit a print statement.
    ///
    /// # Errors
    ///
    /// Returns an error if the expression cannot be evaluated.
    fn visit_print(&mut self, expr: &Expr) -> Result<R, Self::Error>;

    /// Visit an expression statement.
    ///
    /// # Errors
    ///
    /// Returns an error if the expression cannot be evaluated.
    fn visit_expression(&mut self, expr: &Expr) -> Result<R, Self::Error>;

    /// Visit a variable declaration statement.
    ///
    /// # Errors
    ///
    /// Returns an error if the expression cannot be evaluated.
    fn visit_variable_declaration(
        &mut self,
        name: &Token,
        expr: &Option<Expr>,
    ) -> Result<R, Self::Error>;
}
