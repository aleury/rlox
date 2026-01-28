use crate::{
    expr::{Expr, ExprVisitor, Value},
    scanner::Token,
};

pub struct ASTPrinter;

impl ASTPrinter {
    /// Print an expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ASTPrinter` fails to print the expression.
    pub fn print(&mut self, expr: &Expr) -> Result<String, std::convert::Infallible> {
        expr.accept(self)
    }

    fn parenthesize(
        &mut self,
        name: &str,
        exprs: &[&Expr],
    ) -> Result<String, std::convert::Infallible> {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self)?);
        }
        builder.push(')');

        Ok(builder)
    }
}

impl ExprVisitor<String> for ASTPrinter {
    type Error = std::convert::Infallible;

    fn visit_assign(&mut self, name: &Token, expr: &Expr) -> Result<String, Self::Error> {
        let name = format!("assign({})", name.lexeme);
        self.parenthesize(&name, &[expr])
    }

    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<String, Self::Error> {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Result<String, Self::Error> {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal(&mut self, value: &Value) -> Result<String, Self::Error> {
        let result = match value {
            Value::Number(number) => number.to_string(),
            Value::String(string) => format!("\"{string}\""),
            Value::Bool(value) => value.to_string(),
            Value::Nil => "nil".to_string(),
        };
        Ok(result)
    }

    fn visit_variable(&mut self, name: &Token) -> Result<String, Self::Error> {
        Ok(name.lexeme.clone())
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<String, Self::Error> {
        self.parenthesize(&operator.lexeme, &[right])
    }
}
