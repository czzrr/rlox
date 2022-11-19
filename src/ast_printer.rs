use crate::{expr::Expr, token::{Token, Literal}, token_type::TokenType};

pub fn example() {
    let u = Expr::Unary(Token::new(TokenType::Minus, "-".to_owned(), None, 1), Box::new(Expr::Literal(Literal::Double(123.0))));
    let s = Token::new(TokenType::Star, "*".to_owned(), None, 1);
    let g = Expr::Grouping(Box::new(Expr::Literal(Literal::Double(45.67))));
    let e = Expr::Binary(Box::new(u), s, Box::new(g));
    println!("{}", print(&e));
}

pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Binary(left, operator, right) => parenthesize(&operator.lexeme, vec![left, right]),
        Expr::Grouping(expression) => parenthesize("group", vec![expression]),
        Expr::Literal(value) => parenthesize(&value.to_string(), Vec::<&Box<Expr>>::new()),
        Expr::Unary(operator, right) => parenthesize(&operator.lexeme, vec![right]),
    }
}

fn parenthesize<T: AsRef<Expr>>(name: & str, exprs: impl IntoIterator<Item = T>) -> String
{
    let mut builder = String::new();

    builder.push('(');
    builder.push_str(name);
    for expr in exprs {
        builder.push(' ');
        builder.push_str(&print(expr.as_ref()));
    }
    builder.push(')');

    builder
}
