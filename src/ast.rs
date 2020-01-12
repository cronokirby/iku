use std::fmt;

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
    /// A tuple, like (1, 2)
    Tuple(Vec<Litteral>)
}

impl fmt::Display for Litteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Litteral::Str(s) => write!(f, "{}", s),
            Litteral::I64(i) => write!(f, "{}", i),
            Litteral::Bool(b) => write!(f, "{}", b),
            // This code is complicated because we want to print single tuples like (1,)
            Litteral::Tuple(litterals) => {
                write!(f, "(")?;
                let mut iter = litterals.iter();
                if let Some(l) = iter.next() {
                    l.fmt(f)?;
                }
                if let Some(l) = iter.next() {
                    write!(f, ", {}", l)?;
                } else {
                    write!(f, ",")?;
                }
                for l in iter {
                    write!(f, ", {}", l)?;
                }
                write!(f, ")")
            }
        }
    }
}

/// Represents what kind of operations exist
///
/// This also includes operations that return different types, e.g. 2 == 2,
/// which returns a boolean instead of an integer
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    /// Check for equality with ==
    Equal,
    /// Check for inequality with !=
    NotEqual,
    /// Compare with <=
    Leq,
    /// Compare with <
    Less,
    /// Compare with >=
    Geq,
    /// Compare with >
    Greater,
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Modulo
    Mod,
}

/// Represents a short circuiting operation between booleans.
///
/// This is treated differently from standard operators, because the evaluation
/// is short-circuiting.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoolOp {
    /// The && operator
    And,
    /// The || operator
    Or
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
    /// A binary operation between two arguments
    BinOp(Op, Box<Expr>, Box<Expr>),
    /// A conditional short circuiting operation between two arguments
    ConditionalOp(BoolOp, Box<Expr>, Box<Expr>),
    /// An if else expresion, like `if y { 3 } else { 4 }`
    IfElse(Box<Expr>, Vec<Expr>, Vec<Expr>),
    /// The unary negation operator !
    Not(Box<Expr>),
    /// The tuple constructor
    MakeTuple(Vec<Expr>),
    /// A reference to a variable name
    Name(String),
}

/// Instead of being a type itself, this is just a syntactic reference to a type
#[derive(Clone, Debug, PartialEq)]
pub enum TypeName {
    /// A raw name of a type
    Name(String),
    /// A tuple of type names
    Tuple(Vec<TypeName>)
}

/// Represents a function definition.
///
/// Functions have a name, as well as some code to run when they're called.
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    /// The name we can use to call the function
    pub name: String,
    /// The named arguments this function accepts
    pub args: Vec<(String, TypeName)>,
    /// An optionally declared return type
    pub ret: Option<TypeName>,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_works_for_single_tuples() {
        assert_eq!("(1,)", format!("{}", Litteral::Tuple(vec![Litteral::I64(1)])));
    }
}
