use crate::{
    expr::{Expr, ExprVisitor, Literal},
    scanner::Token,
};

pub struct ASTPrinter;

impl ASTPrinter {
    #[must_use]
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');

        builder
    }
}

impl ExprVisitor<String> for ASTPrinter {
    fn visit_literal(&self, value: &Literal) -> String {
        match value {
            Literal::Number(number) => number.to_string(),
            Literal::String(string) => string.clone(),
            Literal::Bool(value) => value.to_string(),
            Literal::Nil => "nil".to_string(),
        }
    }

    fn visit_grouping(&self, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }

    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }
}
