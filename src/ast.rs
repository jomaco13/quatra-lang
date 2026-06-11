use crate::qud::Qud;

#[derive(Debug, Clone)]
pub enum Expr {
    QudLiteral(Qud),
    Var(String),
    BinOp(Box<Expr>, BinOperator, Box<Expr>),
    UnaryOp(UnaryOperator, Box<Expr>),
    FuncCall(String, Vec<Expr>),
    Lambda(Vec<String>, Box<Expr>),
    LetBind(String, Box<Expr>, Box<Expr>),
    Collapse(Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinOperator {
    QAdd,
    QMul,
    QMax,
    QMin,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    QNot,
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub name: String,
    pub args: Vec<String>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<FuncDef>,
    pub main: Expr,
}
