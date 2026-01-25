use crate::{
    expr::{Expr, Literal},
    scanner::{Literal as TokenLiteral, Token, TokenType},
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

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
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

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
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

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
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

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
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
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: Literal::Bool(true),
            });
        }
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: Literal::Bool(false),
            });
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }
        if self.matches(&[TokenType::Number]) {
            let prev = self.previous();
            let Some(TokenLiteral::Number(number)) = prev.literal else {
                unreachable!("expected a number literal")
            };
            return Ok(Expr::Literal {
                value: Literal::Number(number),
            });
        }
        if self.matches(&[TokenType::String]) {
            let prev = self.previous();
            let Some(TokenLiteral::String(string)) = prev.literal else {
                unreachable!("expected a string literal")
            };
            return Ok(Expr::Literal {
                value: Literal::String(string),
            });
        }
        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }
        Err(ParseError::UnexpectedToken {
            token: self.peek(),
            message: "Expect expression".into(),
        })
    }

    fn consume(&mut self, token_type: TokenType, error_message: &str) -> Result<(), ParseError> {
        if self.check(token_type) {
            self.advance();
            return Ok(());
        }
        Err(ParseError::UnexpectedToken {
            token: self.peek(),
            message: error_message.into(),
        })
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn matches(&mut self, token_types: &[TokenType]) -> bool {
        for &token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}
