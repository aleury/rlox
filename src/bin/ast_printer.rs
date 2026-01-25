use rlox::{
    ast_printer::ASTPrinter,
    expr::{Expr, Literal},
    scanner::{Token, TokenType},
};

fn main() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token {
                line: 1,
                lexeme: "-".into(),
                literal: None,
                token_type: TokenType::Minus,
            },
            right: Box::new(Expr::Literal {
                value: Literal::Number(123.0),
            }),
        }),
        operator: Token {
            line: 1,
            lexeme: "*".into(),
            literal: None,
            token_type: TokenType::Star,
        },
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Literal::Number(45.67),
            }),
        }),
    };
    println!("{}", ASTPrinter.print(&expr));
}
