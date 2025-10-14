use crate::parser::expr::*;
use crate::parser::func::Func;
use crate::parser::func_def::FuncDef;
use crate::parser::program::Program;
use crate::parser::stmt::Stmt;
use crate::parser::top_level::TopLevel;
use crate::parser::visitor::Visitor;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Symbol {
    Variable,
    Function { parameters: Vec<String> },
}

pub type SymbolTable = HashMap<String, Symbol>;

#[derive(Debug)]
pub struct Scope {
    pub table: SymbolTable,
    pub children: Vec<Scope>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            table: SymbolTable::new(),
            children: vec![],
        }
    }
}

pub struct SymbolTableBuilder<'a> {
    scope: &'a mut Scope,
}

impl<'a> SymbolTableBuilder<'a> {
    pub fn new(scope: &'a mut Scope) -> Self {
        Self { scope }
    }

    fn add_symbol(&mut self, name: String, symbol: Symbol) {
        self.scope.table.insert(name, symbol);
    }
}

impl<'a> Visitor<()> for SymbolTableBuilder<'a> {
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
        self.add_symbol(
            func_def.name.clone(),
            Symbol::Function {
                parameters: func_def.func.args.clone(),
            },
        );
        func_def.func.accept(self);
    }

    fn visit_func(&mut self, func: &Func) {
        self.scope.children.push(Scope::new());
        let mut sub_visitor = SymbolTableBuilder::new(self.scope.children.last_mut().unwrap());
        for param in &func.args {
            sub_visitor.add_symbol(param.clone(), Symbol::Variable);
        }
        for stmt in &func.body {
            stmt.accept(&mut sub_visitor);
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => expr.accept(self),
            Stmt::Var(vars) => {
                for (name, expr_opt) in vars {
                    self.add_symbol(name.clone(), Symbol::Variable);
                    if let Some(expr) = expr_opt {
                        expr.accept(self);
                    }
                }
            }
            Stmt::If(cond, then_stmt, else_stmt_opt) => {
                cond.accept(self);
                self.scope.children.push(Scope::new());
                let mut then_visitor =
                    SymbolTableBuilder::new(self.scope.children.last_mut().unwrap());
                then_stmt.accept(&mut then_visitor);

                if let Some(else_stmt) = else_stmt_opt {
                    self.scope.children.push(Scope::new());
                    let mut else_visitor =
                        SymbolTableBuilder::new(self.scope.children.last_mut().unwrap());
                    else_stmt.accept(&mut else_visitor);
                }
            }
            Stmt::Block(stmts) => {
                self.scope.children.push(Scope::new());
                let mut sub_visitor =
                    SymbolTableBuilder::new(self.scope.children.last_mut().unwrap());
                for stmt in stmts {
                    stmt.accept(&mut sub_visitor);
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
                self.scope.children.push(Scope::new());
                let mut sub_visitor =
                    SymbolTableBuilder::new(self.scope.children.last_mut().unwrap());
                body.accept(&mut sub_visitor);
            }
            Stmt::While(cond, body) => {
                cond.accept(self);
                self.scope.children.push(Scope::new());
                let mut sub_visitor =
                    SymbolTableBuilder::new(self.scope.children.last_mut().unwrap());
                body.accept(&mut sub_visitor);
            }
            Stmt::DoUntil(body, cond) => {
                self.scope.children.push(Scope::new());
                let mut sub_visitor =
                    SymbolTableBuilder::new(self.scope.children.last_mut().unwrap());
                body.accept(&mut sub_visitor);
                cond.accept(self); // Condition is evaluated in the outer scope
            }
            Stmt::For(init, cond_opt, update_opt, body) => {
                self.scope.children.push(Scope::new());
                let mut sub_visitor =
                    SymbolTableBuilder::new(self.scope.children.last_mut().unwrap());
                if let Some(init_stmt) = init {
                    init_stmt.accept(&mut sub_visitor);
                }
                if let Some(cond_expr) = cond_opt {
                    cond_expr.accept(&mut sub_visitor);
                }
                if let Some(update_stmt) = update_opt {
                    update_stmt.accept(&mut sub_visitor);
                }
                body.accept(&mut sub_visitor);
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
