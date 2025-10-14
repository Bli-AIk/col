use crate::parser::func_def::FuncDef;
use crate::parser::stmt::Stmt;
use crate::parser::visitor::Visitor;

#[derive(Debug, Clone)]
pub enum TopLevel {
    Statement(Stmt),
    Function(FuncDef),
}

impl TopLevel {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_toplevel(self)
    }
}
