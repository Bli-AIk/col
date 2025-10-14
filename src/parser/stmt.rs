use crate::parser::expr::Expr;
use crate::parser::visitor::Visitor;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Var(Vec<(String, Option<Expr>)>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    Block(Vec<Stmt>),
    Return(Option<Expr>),
    Break,
    Continue,
    Repeat(Box<Expr>, Box<Stmt>),
    While(Box<Expr>, Box<Stmt>),
    DoUntil(Box<Stmt>, Box<Expr>),
    For(
        Option<Box<Stmt>>,
        Option<Box<Expr>>,
        Option<Box<Stmt>>,
        Box<Stmt>,
    ),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_stmt(self)
    }
}
