use rlox::{
    ast_printer::ASTPrinter,
    expr::{Expr, Value},
    scanner::{Token, TokenKind},
};

fn main() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token { line: 1, lexeme: "-".into(), kind: TokenKind::Minus },
            right: Box::new(Expr::Literal { value: Value::Number(123.0) }),
        }),
        operator: Token { line: 1, lexeme: "*".into(), kind: TokenKind::Star },
        right: Box::new(Expr::Grouping { expression: Box::new(Expr::Literal { value: Value::Number(45.67) }) }),
    };
    println!("{}", ASTPrinter.print(&expr).expect("print ast"));
}
