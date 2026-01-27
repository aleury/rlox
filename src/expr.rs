use std::fmt::{Display, Formatter};

use crate::scanner::Token;

#[derive(Debug)]
pub enum Expr {
    Literal {
        value: Value,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
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
    pub fn accept<T, Visitor>(&self, visitor: &Visitor) -> Result<T, Visitor::Error>
    where
        Visitor: ExprVisitor<T>,
    {
        match self {
            Expr::Literal { value } => visitor.visit_value(value),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
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

    /// Visit a literal value.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the literal value.
    fn visit_value(&self, value: &Value) -> Result<T, Self::Error>;

    /// Visit a grouping expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the grouping expression.
    fn visit_grouping(&self, expression: &Expr) -> Result<T, Self::Error>;

    /// Visit a unary expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the unary expression.
    fn visit_unary(&self, operator: &Token, right: &Expr) -> Result<T, Self::Error>;

    /// Visit a binary expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ExprVisitor` fails to visit the binary expression.
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<T, Self::Error>;
}
