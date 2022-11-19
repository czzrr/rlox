use crate::{token::{Token, Literal}, expr::Expr, token_type::TokenType, error_handler::ErrorHandler};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum InterpErr {
    #[error("Operand must be a number.")]
    OpMustBeNum,
    #[error("Operands must be numbers.")]
    OpsMustBeNums,
    #[error("Operands must be two numbers or two strings.")]
    OpsMustBeNumsOrStrs,
    //#[error("Undefined variable '{0}'.")]
    //UndefVar(String)
}

type InterpResult = Result<Literal, (Token, InterpErr)>;

pub fn interpret(expr: Expr, error_handler: &mut ErrorHandler) {
    match evaluate(expr) {
        Ok(literal) => println!("{}", literal.to_string()),
        Err((token, interp_err)) => error_handler.runtime_error(&token, interp_err),
    }
    

}

pub fn evaluate(expr: Expr) -> InterpResult {
    match expr {
        Expr::Literal(literal) => Ok(literal),
        Expr::Grouping(expression) => evaluate(*expression),
        Expr::Unary(operator, right) => {
            let right = evaluate(*right)?;

            match (operator.ty, right) {
                (TokenType::Minus, Literal::Double(n)) => Ok(Literal::Double(-n)),
                (TokenType::Bang, right) => Ok(Literal::Bool(is_truthy(&right))),
                _ => Err((operator, InterpErr::OpMustBeNum))
            }
        },
        Expr::Binary(left, operator, right) => {
            let left = evaluate(*left)?;
            let right = evaluate(*right)?;
            match operator.ty {
                TokenType::Minus => {
                    subtract(&left, &right).ok_or((operator, InterpErr::OpsMustBeNums))
                }
                TokenType::Slash => {
                    divide(&left, &right).ok_or((operator, InterpErr::OpsMustBeNums))
                }
                TokenType::Star => {
                    multiply(&left, &right).ok_or((operator, InterpErr::OpsMustBeNums))
                }
                TokenType::Plus => {
                    add(&left, &right).ok_or((operator, InterpErr::OpsMustBeNumsOrStrs))
                }
                TokenType::Greater => comparison(&left, &right, &|l, r| l > r)
                    .ok_or((operator, InterpErr::OpsMustBeNums)),
                TokenType::GreaterEqual => comparison(&left, &right, &|l, r| l >= r)
                    .ok_or((operator, InterpErr::OpsMustBeNums)),
                TokenType::Less => comparison(&left, &right, &|l, r| l < r)
                    .ok_or((operator, InterpErr::OpsMustBeNums)),
                TokenType::LessEqual => comparison(&left, &right, &|l, r| l <= r)
                    .ok_or((operator, InterpErr::OpsMustBeNums)),
                TokenType::BangEqual => Ok(Literal::Bool(left != right)),
                TokenType::EqualEqual => Ok(Literal::Bool(left == right)),
                _ => Ok(Literal::Nil),
            }
        },
        _ => todo!()
    }
}

fn is_truthy(lit: &Literal) -> bool {
    match lit {
        Literal::Nil => false,
        Literal::Bool(val) => val.to_owned(),
        _ => true,
    }
}

fn subtract(left: &Literal, right: &Literal) -> Option<Literal> {
    match (left, right) {
        (Literal::Double(l), Literal::Double(r)) => Some(Literal::Double(l - r)),
        _ => None,
    }
}

fn divide(left: &Literal, right: &Literal) -> Option<Literal> {
    match (left, right) {
        (Literal::Double(l), Literal::Double(r)) => Some(Literal::Double(l / r)),
        _ => None,
    }
}

fn multiply(left: &Literal, right: &Literal) -> Option<Literal> {
    match (left, right) {
        (Literal::Double(l), Literal::Double(r)) => Some(Literal::Double(l * r)),
        _ => None,
    }
}

fn add(left: &Literal, right: &Literal) -> Option<Literal> {
    match (left, right) {
        (Literal::Double(l), Literal::Double(r)) => Some(Literal::Double(l + r)),
        (Literal::Str(l), Literal::Str(r)) => {
            let mut res = l.clone();
            res.push_str(r);
            Some(Literal::Str(res))
        }
        _ => None,
    }
}

fn comparison(left: &Literal, right: &Literal, comp: &dyn Fn(f64, f64) -> bool) -> Option<Literal> {
    match (left, right) {
        (Literal::Double(l), Literal::Double(r)) => Some(Literal::Bool(comp(*l, *r))),
        _ => None,
    }
}

