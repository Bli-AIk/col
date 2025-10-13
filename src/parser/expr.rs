#[derive(Debug, Clone)]
pub struct Func {
    pub args: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Var(Vec<(String, Option<Expr>)>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    Block(Vec<Stmt>),
}


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
}
