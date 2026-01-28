use std::fmt::{Display, Formatter};

use crate::scanner::Token;

#[derive(Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Var {
        name: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    /// Accept a visitor.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the expression.
    pub fn accept<T, Visitor>(&self, visitor: &mut Visitor) -> Result<T, Visitor::Error>
    where
        Visitor: ExprVisitor<T>,
    {
        match self {
            Expr::Assign { name, value } => visitor.visit_assign(name, value),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Var { name } => visitor.visit_variable(name),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{num}"),
            Value::String(str) => write!(f, "\"{str}\""),
            Value::Bool(bool) => write!(f, "{bool}"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

pub trait ExprVisitor<T> {
    type Error;

    /// Visit an assignment expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the assignment expression.
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> Result<T, Self::Error>;

    /// Visit a binary expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the binary expression.
    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<T, Self::Error>;

    /// Visit a grouping expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the grouping expression.
    fn visit_grouping(&mut self, expression: &Expr) -> Result<T, Self::Error>;

    /// Visit a literal value.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the literal value.
    fn visit_literal(&mut self, value: &Value) -> Result<T, Self::Error>;

    /// Visit a variable expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the variable expression.
    fn visit_variable(&mut self, name: &Token) -> Result<T, Self::Error>;

    /// Visit a unary expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the unary expression.
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<T, Self::Error>;
}
