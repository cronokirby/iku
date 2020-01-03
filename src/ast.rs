/// Represents an expression in the Iku language.
///
/// Expressions can be evaluated to some kind of value.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Call a function by name, with arguments expression
    Call(String, Box<Expr>),
    I32(i32),
    Str(String),
}

/// Represents the full abstract syntax tree of an Iku program.
///
/// At the moment, a program is synonymous with a single file. This will
/// probably change at some point.
#[derive(Clone, Debug, PartialEq)]
pub enum AST {
    FuncMain(Expr),
}

