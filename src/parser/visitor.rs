use super::expr::*;
use crate::parser::func::Func;
use crate::parser::func_def::FuncDef;
use crate::parser::program::Program;
use crate::parser::stmt::Stmt;
use crate::parser::top_level::TopLevel;
pub mod dead_code_detector;
pub mod performance_warner;
pub mod symbol_table_builder;
pub mod symbol_table_builder_tests;
pub mod type_checker;

pub trait Visitor<T> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_toplevel(&mut self, toplevel: &TopLevel) -> T;
    fn visit_func_def(&mut self, func_def: &FuncDef) -> T;
    fn visit_func(&mut self, func: &Func) -> T;
    fn visit_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
}
