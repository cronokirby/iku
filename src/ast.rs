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
    /// A boolean litteral
    Bool(bool),
}

/// Represents what kind of operations exist
///
/// This also includes operations that return different types, e.g. 2 == 2,
/// which returns a boolean instead of an integer
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    /// Check for equality with ==
    Equal,
    /// Compare with <=
    Leq,
    /// Compare with <
    Less,
    /// Compare with >=
    Geq,
    /// Compare with >
    Greater,
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
    Call(String, Vec<Expr>),
    /// Represents the use of a litteral as an expression
    Litt(Litteral),
    /// A variable declaration, like `x := 2`
    Declare(String, Box<Expr>),
    /// A variable assignment, like `x = 3`
    Assign(String, Box<Expr>),
    /// A block of expressions
    Block(Vec<Expr>),
    BinOp(Op, Box<Expr>, Box<Expr>),
    /// A reference to a variable name
    Name(String),
}

/// Represents a function definition.
///
/// Functions have a name, as well as some code to run when they're called.
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    /// The name we can use to call the function
    pub name: String,
    /// The named arguments this function accepts
    pub args: Vec<String>,
    /// The series of expressions making up the body of this function
    pub body: Vec<Expr>,
}

/// Represents the full abstract syntax tree of an Iku program.
///
/// At the moment, a program is synonymous with a single file. This will
/// probably change at some point.
#[derive(Clone, Debug, PartialEq)]
pub struct AST {
    pub functions: Vec<Function>,
}
