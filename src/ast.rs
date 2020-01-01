#[derive(Debug)]
pub enum Expr {
    Print(Box<Expr>),
    I32(i32)
}

#[derive(Debug)]
pub enum AST {
    FuncMain(Expr)
}
