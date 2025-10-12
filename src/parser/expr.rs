use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum Expr {
    Number(f64),
    String(String),
    Var(String),
    True(bool),
    False(bool),
    Addition(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Positive(Box<Expr>),
    Negative(Box<Expr>),
    Paren(Box<Expr>),
}
