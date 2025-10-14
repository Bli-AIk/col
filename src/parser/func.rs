use crate::parser::stmt::Stmt;
use crate::parser::visitor::Visitor;

#[derive(Debug, Clone)]
pub struct Func {
    pub args: Vec<String>,
    pub body: Vec<Stmt>,
}

impl Func {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_func(self)
    }
}
