#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Assign(String, Expr),
    Expr(Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),      // condition, then-block, else-block
    While(Expr, Vec<Stmt>),               // condition, body
    DoWhile(Vec<Stmt>, Expr),             // body, condition
    For(String, Expr, Expr, Expr, Vec<Stmt>), // var, start, cond, step, body
    FnDecl(String, Vec<String>, Vec<Stmt>),   // name, params, body
    Return(Expr),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Variable(String),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Gt,      // Changed from Greater to Gt to match parser usage
    Lt,      // Changed from Less to Lt
    Eq,      // Changed from Equal to Eq
    Neq,     // Changed from NotEqual to Neq
}