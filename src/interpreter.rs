use crate::{
    environment::Environment,
    expr::{Expr, ExprVisitor, Value},
    scanner::{Token, TokenKind},
    stmt::{Stmt, StmtVisitor},
};

macro_rules! error {
    ($op:expr, $msg:expr) => {
        Err(RuntimeError {
            token: $op.clone(),
            message: $msg.to_string(),
        })
    };
    ($op:expr, $msg:expr, $value:expr) => {
        Err(RuntimeError {
            token: $op.clone(),
            message: format!("{}, got {} ({})", $msg, $value, type_of(&$value),),
        })
    };
    ($op:expr, $msg:expr, $left:expr, $right:expr) => {
        Err(RuntimeError {
            token: $op.clone(),
            message: format!(
                "{}, got {} ({}) and {} ({})",
                $msg,
                $left,
                type_of(&$left),
                $right,
                type_of(&$right),
            ),
        })
    };
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Interpreter {
    /// Interpret a list of statements and return a result or an error.
    ///
    /// # Errors
    ///
    /// - `RuntimeError`: If an error occurs during interpretation.
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(&statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), RuntimeError> {
        statement.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }
}

impl StmtVisitor<()> for Interpreter {
    type Error = RuntimeError;

    fn visit_print(&mut self, expr: &Expr) -> Result<(), Self::Error> {
        let value = self.evaluate(expr)?;
        println!("{value}");
        Ok(())
    }

    fn visit_expression(&mut self, expr: &Expr) -> Result<(), Self::Error> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_variable_declaration(
        &mut self,
        name: &Token,
        expr: &Option<Expr>,
    ) -> Result<(), Self::Error> {
        let value = match expr {
            Some(expr) => self.evaluate(expr)?,
            None => Value::Nil,
        };
        self.env.define(name.lexeme.clone(), value);
        Ok(())
    }
}

impl ExprVisitor<Value> for Interpreter {
    type Error = RuntimeError;

    fn visit_assign(&mut self, name: &Token, value: &Expr) -> Result<Value, Self::Error> {
        let value = self.evaluate(value)?;
        self.env
            .assign(&name.lexeme, value.clone())
            .map_err(|err| RuntimeError {
                token: name.clone(),
                message: err,
            })?;
        Ok(value)
    }

    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, Self::Error> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
        match &operator.kind {
            TokenKind::Plus => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{a}{b}"))),
                _ => error!(
                    operator,
                    "Operands must be two numbers or two strings", left, right
                ),
            },
            TokenKind::Minus => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => error!(operator, "Operands must be numbers", left, right),
            },
            TokenKind::Slash => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
                _ => error!(operator, "Operands must be numbers", left, right),
            },
            TokenKind::Star => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                _ => error!(operator, "Operands must be numbers", left, right),
            },
            TokenKind::Greater => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                _ => error!(operator, "Operands must be numbers", left, right),
            },
            TokenKind::GreaterEqual => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                _ => error!(operator, "Operands must be numbers", left, right),
            },
            TokenKind::Less => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                _ => error!(operator, "Operands must be numbers", left, right),
            },
            TokenKind::LessEqual => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                _ => error!(operator, "Operands must be numbers", left, right),
            },
            TokenKind::BangEqual => Ok(Value::Bool(!is_equal(&left, &right))),
            TokenKind::EqualEqual => Ok(Value::Bool(is_equal(&left, &right))),
            _ => error!(operator, "Unknown binary operator"),
        }
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Result<Value, Self::Error> {
        self.evaluate(expression)
    }

    fn visit_literal(&mut self, value: &Value) -> Result<Value, Self::Error> {
        Ok(value.clone())
    }

    fn visit_variable(&mut self, name: &Token) -> Result<Value, Self::Error> {
        let Some(value) = self.env.get(&name.lexeme) else {
            return Err(RuntimeError {
                token: name.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme),
            });
        };
        Ok(value.clone())
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<Value, Self::Error> {
        let right = self.evaluate(right)?;

        match (&operator.kind, &right) {
            (TokenKind::Minus, Value::Number(num)) => Ok(Value::Number(-num)),
            (TokenKind::Bang, value) => Ok(Value::Bool(!is_truthy(value))),
            _ => error!(operator, "Operand must be a number", right),
        }
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Bool(bool) => *bool,
        _ => true,
    }
}

fn is_equal(a: &Value, b: &Value) -> bool {
    #[allow(clippy::float_cmp)]
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        _ => false,
    }
}

fn type_of(value: &Value) -> &'static str {
    match value {
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Bool(_) => "boolean",
        Value::Nil => "nil",
    }
}
