use crate::{
    expr::{Expr, ExprVisitor, Value},
    scanner::{Token, TokenKind},
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

pub struct Interpreter;

impl Interpreter {
    /// Interpret an expression and return its value or an error.
    ///
    /// # Errors
    ///
    /// - `RuntimeError`: If an error occurs during interpretation.
    pub fn interpret(&self, expression: &Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(expression)?;
        println!("{value}");
        Ok(())
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }
}

impl ExprVisitor<Value> for Interpreter {
    type Error = RuntimeError;

    fn visit_value(&self, value: &Value) -> Result<Value, Self::Error> {
        Ok(value.clone())
    }

    fn visit_grouping(&self, expression: &Expr) -> Result<Value, Self::Error> {
        self.evaluate(expression)
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> Result<Value, Self::Error> {
        let right = self.evaluate(right)?;

        match (&operator.kind, &right) {
            (TokenKind::Minus, Value::Number(num)) => Ok(Value::Number(-num)),
            (TokenKind::Bang, value) => Ok(Value::Bool(!is_truthy(value))),
            _ => error!(operator, "Operand must be a number", right),
        }
    }

    fn visit_binary(
        &self,
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
