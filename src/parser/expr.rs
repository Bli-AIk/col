use crate::parser::visitor::Visitor;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    True(bool),
    False(bool),
    Null,
    Identifier(String),
    Call(String, Vec<Expr>),
    Addition(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Percent(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    BitNot(Box<Expr>),
    Positive(Box<Expr>),
    Negative(Box<Expr>),
    Paren(Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    EqualEqual(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    BitAnd(Box<Expr>, Box<Expr>),
    BitXor(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    PlusEqual(Box<Expr>, Box<Expr>),
    MinusEqual(Box<Expr>, Box<Expr>),
    StarEqual(Box<Expr>, Box<Expr>),
    SlashEqual(Box<Expr>, Box<Expr>),
    PercentEqual(Box<Expr>, Box<Expr>),
    PreIncrement(Box<Expr>),
    PostIncrement(Box<Expr>),
    PreDecrement(Box<Expr>),
    PostDecrement(Box<Expr>),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_expr(self)
    }
}
