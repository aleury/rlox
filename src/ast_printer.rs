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
    pub fn print(&self, expr: &Expr) -> Result<String, std::convert::Infallible> {
        expr.accept(self)
    }

    fn parenthesize(
        &self,
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

    fn visit_value(&self, value: &Value) -> Result<String, Self::Error> {
        let result = match value {
            Value::Number(number) => number.to_string(),
            Value::String(string) => format!("\"{string}\""),
            Value::Bool(value) => value.to_string(),
            Value::Nil => "nil".to_string(),
        };
        Ok(result)
    }

    fn visit_grouping(&self, expression: &Expr) -> Result<String, Self::Error> {
        self.parenthesize("group", &[expression])
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> Result<String, Self::Error> {
        self.parenthesize(&operator.lexeme, &[right])
    }

    fn visit_binary(
        &self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<String, Self::Error> {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_variable(&self, name: &Token) -> Result<String, Self::Error> {
        Ok(name.lexeme.clone())
    }
}
