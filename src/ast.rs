/// Represents a litteral value in the language
///
/// Litterals can be thought of as the fully evaluated result of an expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Litteral {
    /// Represents a string litteral, like `"hello"`
    Str(String),
    /// Represents an integer litteral, like `333`.
    ///
    /// Right now all string litterals are 64 bit signed integers,
    /// but we might want litterals to be big nums at some point.
    I64(i64),
}

/// Represents an expression in the Iku language.
///
/// Expressions can be evaluated to some kind of value.
///
/// Expression are also synonymous with statements, as all statements
/// resolve to some value
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Call a function by name, with arguments expression
    Call(String, Box<Expr>),
    /// Represents the use of a litteral as an expression
    Litt(Litteral),
    /// A variable declaration, like `x := 2`
    Declare(String, Box<Expr>),
}

/// Represents the full abstract syntax tree of an Iku program.
///
/// At the moment, a program is synonymous with a single file. This will
/// probably change at some point.
#[derive(Clone, Debug, PartialEq)]
pub enum AST {
    /// A main function containing a series of expressions
    FuncMain(Vec<Expr>),
}
