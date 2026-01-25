use crate::scanner::Token;

#[derive(Debug)]
pub enum Expr {
    Literal {
        value: Literal,
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
    pub fn accept<T, Visitor>(&self, visitor: &Visitor) -> T
    where
        Visitor: ExprVisitor<T>,
    {
        match self {
            Expr::Literal { value } => visitor.visit_literal(value),
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

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

pub trait ExprVisitor<T> {
    fn visit_literal(&self, value: &Literal) -> T;
    fn visit_grouping(&self, expression: &Expr) -> T;
    fn visit_unary(&self, operator: &Token, right: &Expr) -> T;
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> T;
}
