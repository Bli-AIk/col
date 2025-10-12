use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum Expr {
    Num(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pos(Box<Expr>),
    Neg(Box<Expr>),
    Paren(Box<Expr>),
}
