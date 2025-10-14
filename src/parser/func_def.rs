use crate::parser::func::Func;
use crate::parser::visitor::Visitor;

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub name: String,
    pub func: Func,
}

impl FuncDef {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_func_def(self)
    }
}
