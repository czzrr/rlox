use crate::{
    environment::Environment,
    error_handler::ErrorHandler,
    expr::Expr,
    stmt::Stmt,
    token::{Literal, Token},
    token_type::TokenType,
};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum InterpErr {
    #[error("Operand must be a number.")]
    OpMustBeNum,
    #[error("Operands must be numbers.")]
    OpsMustBeNums,
    #[error("Operands must be two numbers or two strings.")]
    OpsMustBeNumsOrStrs,
    #[error("Undefined variable '{0}'.")]
    UndefVar(String)
}

type InterpResult = Result<Literal, (Token, InterpErr)>;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>, error_handler: &mut ErrorHandler) {
        for statement in statements {
            match self.execute_stmt(statement) {
                Err((token, err)) => error_handler.runtime_error(&token, err),
                _ => (),
            }
        }
    }

    pub fn execute_stmt(&mut self, stmt: Stmt) -> InterpResult {
        match stmt {
            Stmt::ExprStmt(expr) => self.evaluate_expr(expr),
            Stmt::PrintStmt(expr) => {
                let val = self.evaluate_expr(expr);
                match val {
                    Ok(ref lit) => {
                        println!("{}", lit.to_string());
                        val
                    }
                    _ => val,
                }
            },
            Stmt::Var(token, initializer) => {
                if let Some(initializer) = initializer {
                    let value = self.evaluate_expr(initializer)?;
                    self.environment.define(token.lexeme, value);
                } else {
                    self.environment.define(token.lexeme, Literal::Nil);
                }
                Ok(Literal::Nil)
            },
            Stmt::Block(statements) => {
                self.execute_block(statements, Environment::new_enclosing(Box::new(self.environment.clone())))
            }
            Stmt::If(cond, if_branch, else_branch) => {
                if Self::is_truthy(&self.evaluate_expr(cond)?) {
                    self.execute_stmt(*if_branch)
                } else if let Some(eb) = else_branch {
                    self.execute_stmt(*eb)
                } else {
                    Ok(Literal::Nil)
                }
            }
        }
    }

    pub fn evaluate_expr(&mut self, expr: Expr) -> InterpResult {
        match expr {
            Expr::Literal(literal) => Ok(literal),
            Expr::Grouping(expression) => self.evaluate_expr(*expression),
            Expr::Unary(operator, right) => {
                let right = self.evaluate_expr(*right)?;

                match (operator.ty, right) {
                    (TokenType::Minus, Literal::Double(n)) => Ok(Literal::Double(-n)),
                    (TokenType::Bang, right) => Ok(Literal::Bool(Self::is_truthy(&right))),
                    _ => Err((operator, InterpErr::OpMustBeNum)),
                }
            }
            Expr::Binary(left, operator, right) => {
                let left = self.evaluate_expr(*left)?;
                let right = self.evaluate_expr(*right)?;
                match operator.ty {
                    TokenType::Minus => {
                        Self::subtract(&left, &right).ok_or((operator, InterpErr::OpsMustBeNums))
                    }
                    TokenType::Slash => {
                        Self::divide(&left, &right).ok_or((operator, InterpErr::OpsMustBeNums))
                    }
                    TokenType::Star => {
                        Self::multiply(&left, &right).ok_or((operator, InterpErr::OpsMustBeNums))
                    }
                    TokenType::Plus => {
                        Self::add(&left, &right).ok_or((operator, InterpErr::OpsMustBeNumsOrStrs))
                    }
                    TokenType::Greater => Self::comparison(&left, &right, &|l, r| l > r)
                        .ok_or((operator, InterpErr::OpsMustBeNums)),
                    TokenType::GreaterEqual => Self::comparison(&left, &right, &|l, r| l >= r)
                        .ok_or((operator, InterpErr::OpsMustBeNums)),
                    TokenType::Less => Self::comparison(&left, &right, &|l, r| l < r)
                        .ok_or((operator, InterpErr::OpsMustBeNums)),
                    TokenType::LessEqual => Self::comparison(&left, &right, &|l, r| l <= r)
                        .ok_or((operator, InterpErr::OpsMustBeNums)),
                    TokenType::BangEqual => Ok(Literal::Bool(left != right)),
                    TokenType::EqualEqual => Ok(Literal::Bool(left == right)),
                    _ => Ok(Literal::Nil),
                }
            },
            Expr::Variable(token) => {
                if let Some(value) = self.environment.get(&token) {
                    Ok(value.clone())
                } else {
                    let s = token.lexeme.to_owned();
                    Err((token, InterpErr::UndefVar(s)))
                }
            },
            Expr::Assign(name, value) => {
                let value = self.evaluate_expr(*value)?;
                if self.environment.assign(&name, &value) {
                    Ok(value)
                } else {
                    let s = name.lexeme.clone();
                    Err((name, InterpErr::UndefVar(s)))
                }
            },
            Expr::Logical(left, op, right) => {
                let left = self.evaluate_expr(*left)?;
                match op.ty {
                    TokenType::Or if Self::is_truthy(&left) => Ok(left),
                    TokenType::And if !Self::is_truthy(&left) => Ok(left),
                    _ => self.evaluate_expr(*right)
                }
            }
        }
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, environment: Environment) -> InterpResult {
        let previous = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            self.execute_stmt(statement)?;
        }
        self.environment = previous;
        Ok(Literal::Nil)
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

    fn comparison(
        left: &Literal,
        right: &Literal,
        comp: &dyn Fn(f64, f64) -> bool,
    ) -> Option<Literal> {
        match (left, right) {
            (Literal::Double(l), Literal::Double(r)) => Some(Literal::Bool(comp(*l, *r))),
            _ => None,
        }
    }
}
