use crate::parser::expr::*;
use crate::parser::func::Func;
use crate::parser::func_def::FuncDef;
use crate::parser::program::Program;
use crate::parser::stmt::Stmt;
use crate::parser::top_level::TopLevel;
use crate::parser::visitor::Visitor;

pub struct SymbolTableBuilder;

impl Visitor<()> for SymbolTableBuilder {
    fn visit_program(&mut self, program: &Program) {
        for toplevel in &program.body {
            toplevel.accept(self);
        }
    }

    fn visit_toplevel(&mut self, toplevel: &TopLevel) {
        match toplevel {
            TopLevel::Statement(stmt) => stmt.accept(self),
            TopLevel::Function(func_def) => func_def.accept(self),
        }
    }

    fn visit_func_def(&mut self, func_def: &FuncDef) {
        // TODO: Add function to symbol table
        func_def.func.accept(self);
    }

    fn visit_func(&mut self, func: &Func) {
        // TODO: Add arguments to symbol table
        for stmt in &func.body {
            stmt.accept(self);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => expr.accept(self),
            Stmt::Var(vars) => {
                for (name, expr_opt) in vars {
                    // TODO: Add variable to symbol table
                    if let Some(expr) = expr_opt {
                        expr.accept(self);
                    }
                }
            }
            Stmt::If(cond, then_stmt, else_stmt_opt) => {
                cond.accept(self);
                then_stmt.accept(self);
                if let Some(else_stmt) = else_stmt_opt {
                    else_stmt.accept(self);
                }
            }
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    stmt.accept(self);
                }
            }
            Stmt::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    expr.accept(self);
                }
            }
            Stmt::Break => {}
            Stmt::Continue => {}
            Stmt::Repeat(count, body) => {
                count.accept(self);
                body.accept(self);
            }
            Stmt::While(cond, body) => {
                cond.accept(self);
                body.accept(self);
            }
            Stmt::DoUntil(body, cond) => {
                body.accept(self);
                cond.accept(self);
            }
            Stmt::For(init, cond_opt, update_opt, body) => {
                if let Some(init_stmt) = init {
                    init_stmt.accept(self);
                }
                if let Some(cond_expr) = cond_opt {
                    cond_expr.accept(self);
                }
                if let Some(update_stmt) = update_opt {
                    update_stmt.accept(self);
                }
                body.accept(self);
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        // Most expressions that contain other expressions need to be recursively visited.
        match expr {
            Expr::Call(_, args) => {
                for arg in args {
                    arg.accept(self);
                }
            }
            Expr::Addition(l, r)
            | Expr::Subtraction(l, r)
            | Expr::Multiplication(l, r)
            | Expr::Division(l, r)
            | Expr::Percent(l, r)
            | Expr::Greater(l, r)
            | Expr::GreaterEqual(l, r)
            | Expr::Less(l, r)
            | Expr::LessEqual(l, r)
            | Expr::EqualEqual(l, r)
            | Expr::NotEqual(l, r)
            | Expr::BitAnd(l, r)
            | Expr::BitXor(l, r)
            | Expr::BitOr(l, r)
            | Expr::And(l, r)
            | Expr::Xor(l, r)
            | Expr::Or(l, r)
            | Expr::Equal(l, r)
            | Expr::PlusEqual(l, r)
            | Expr::MinusEqual(l, r)
            | Expr::StarEqual(l, r)
            | Expr::SlashEqual(l, r)
            | Expr::PercentEqual(l, r) => {
                l.accept(self);
                r.accept(self);
            }
            Expr::Not(e)
            | Expr::BitNot(e)
            | Expr::Positive(e)
            | Expr::Negative(e)
            | Expr::Paren(e)
            | Expr::PreIncrement(e)
            | Expr::PostIncrement(e)
            | Expr::PreDecrement(e)
            | Expr::PostDecrement(e) => e.accept(self),
            Expr::Ternary(cond, then_expr, else_expr) => {
                cond.accept(self);
                then_expr.accept(self);
                else_expr.accept(self);
            }
            // Atoms have no children to visit
            Expr::Number(_)
            | Expr::String(_)
            | Expr::True(_)
            | Expr::False(_)
            | Expr::Null
            | Expr::Identifier(_) => {}
        }
    }
}
