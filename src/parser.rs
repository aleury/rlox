use crate::{
    expr::{Expr, Literal},
    scanner::{Token, TokenKind},
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { token: Token, message: String },
}

pub struct Parser<'a> {
    current: usize,
    tokens: &'a [Token],
}

impl<'a> Parser<'a> {
    #[must_use]
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { current: 0, tokens }
    }

    /// Parse the tokens into an expression.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the tokens cannot be parsed into an expression.
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while matches!(
            self.peek().kind,
            TokenKind::BangEqual | TokenKind::EqualEqual
        ) {
            let operator = self.advance();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while matches!(
            self.peek().kind,
            TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual
        ) {
            let operator = self.advance();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while matches!(self.peek().kind, TokenKind::Minus | TokenKind::Plus) {
            let operator = self.advance();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while matches!(self.peek().kind, TokenKind::Slash | TokenKind::Star) {
            let operator = self.advance();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if matches!(self.peek().kind, TokenKind::Bang | TokenKind::Minus) {
            let operator = self.advance();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance();
        match token.kind {
            TokenKind::True => Ok(Expr::Literal {
                value: Literal::Bool(true),
            }),
            TokenKind::False => Ok(Expr::Literal {
                value: Literal::Bool(false),
            }),
            TokenKind::Nil => Ok(Expr::Literal {
                value: Literal::Nil,
            }),
            TokenKind::Number(number) => Ok(Expr::Literal {
                value: Literal::Number(number),
            }),
            TokenKind::String(string) => Ok(Expr::Literal {
                value: Literal::String(string),
            }),
            TokenKind::LeftParen => {
                let expr = self.expression()?;
                self.consume(&TokenKind::RightParen, "Expect ')' after expression")?;
                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }
            _ => Err(ParseError::UnexpectedToken {
                token,
                message: "Expect expression".into(),
            }),
        }
    }

    fn consume(&mut self, token_type: &TokenKind, error_message: &str) -> Result<(), ParseError> {
        if self.peek().kind == *token_type {
            self.advance();
            return Ok(());
        }
        Err(ParseError::UnexpectedToken {
            token: self.peek(),
            message: error_message.into(),
        })
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].clone()
    }

    fn at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }
}
