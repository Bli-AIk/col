use crate::parser::top_level::TopLevel;
use crate::parser::visitor::Visitor;

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<TopLevel>,
}

impl Program {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_program(self)
    }
}
