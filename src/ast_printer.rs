use crate::expr::{Expr, Value};

pub struct ASTPrinter;

impl ASTPrinter {
    /// Print an expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ASTPrinter` fails to print the expression.
    pub fn print(&mut self, expr: &Expr) -> Result<String, std::convert::Infallible> {
        match expr {
            Expr::Assign { name, value } => {
                let name = format!("assign({})", name.lexeme);
                self.parenthesize(&name, &[value])
            }
            Expr::Binary { left, operator, right } => self.parenthesize(&operator.lexeme, &[left, right]),
            Expr::Grouping { expression } => self.parenthesize("group", &[expression]),
            Expr::Literal { value } => match value {
                Value::Number(number) => Ok(number.to_string()),
                Value::String(string) => Ok(format!("\"{string}\"")),
                Value::Bool(value) => Ok(value.to_string()),
                Value::Nil => Ok("nil".to_string()),
            },
            Expr::Var { name } => Ok(name.lexeme.clone()),
            Expr::Unary { operator, right } => self.parenthesize(&operator.lexeme, &[right]),
        }
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> Result<String, std::convert::Infallible> {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&self.print(expr)?);
        }
        builder.push(')');

        Ok(builder)
    }
}
