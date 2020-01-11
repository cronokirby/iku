use crate::ast::*;
use std::collections::HashMap;

// The value we use when we have nothing to return
// Eventually this will be an empty tuple
const VOID: Litteral = Litteral::I64(0);

/// Represents the contextual abilities an interpreter needs.
///
/// This is made into a trait to allow us to abstract over these effects,
/// allowing us to easily test and use the interpreter.
pub trait Context {
    /// Print a piece of data directly, without adding newlines or anything
    fn print(&mut self, data: &str);
}

/// This struct implements the context trait by actually executing effects.
#[derive(Clone, Copy, Debug, Default)]
pub struct RealContext;

impl Context for RealContext {
    fn print(&mut self, data: &str) {
        print!("{}", data);
    }
}

/// Represents the type of error produced by an interpreter
///
/// These kinds of errors are caused by the user giving us bad code.
/// The interpreter can also panic, if the programmer writing the interpreter
/// made a mistake.
#[derive(Clone, Debug, PartialEq)]
pub struct InterpreterError {
    message: String,
}

impl From<String> for InterpreterError {
    fn from(message: String) -> Self {
        InterpreterError { message }
    }
}

impl<'a> From<&'a str> for InterpreterError {
    fn from(message: &'a str) -> Self {
        InterpreterError {
            message: String::from(message),
        }
    }
}

/// A wrapper type for results that fail in an interpreter
pub type InterpreterResult<T> = Result<T, InterpreterError>;

fn fail<T, S: Into<String>>(message: S) -> InterpreterResult<T> {
    Err(InterpreterError {
        message: message.into(),
    })
}

#[derive(Debug)]
struct Scope {
    // If a scope is nested, then it has access to its parent scopes,
    // otherwise it's detached from those scopes. When calling a function,
    // that function has a completely new scope, whereas an if statement has
    // access to the surrounding scope.
    nested: bool,
    // The variable definitions in this specific scope
    vars: HashMap<String, Litteral>,
}

impl Scope {
    fn new(nested: bool) -> Self {
        Scope {
            nested,
            vars: HashMap::new(),
        }
    }

    // Try and access a variable in this specific scope
    fn get(&self, name: &str) -> Option<&Litteral> {
        self.vars.get(name)
    }

    // Insert a new variable, or replace an existing one
    fn insert<S: Into<String>>(&mut self, name: S, value: Litteral) {
        self.vars.insert(name.into(), value);
    }
}

// This allows us to handle lexical scoping
#[derive(Debug)]
struct Scopes {
    scopes: Vec<Scope>,
}

impl Scopes {
    fn new() -> Self {
        Scopes { scopes: Vec::new() }
    }

    // Enter a new scope
    fn enter(&mut self, nested: bool) {
        self.scopes.push(Scope::new(nested));
    }

    // Exit a scope
    fn exit(&mut self) {
        self.scopes.pop();
    }

    // Get the value of a variable
    fn get(&self, name: &str) -> Option<&Litteral> {
        let mut res = None;
        for scope in self.scopes.iter().rev() {
            let found = scope.get(name);
            if found.is_some() {
                res = found;
                break;
            }
            if !scope.nested {
                break;
            }
        }
        res
    }

    // Get the value of a variable
    // Returns whether or not we managed to find a variable to set.
    fn set<S: Into<String>>(&mut self, name: S, value: Litteral) -> bool {
        let name = name.into();
        for scope in self.scopes.iter_mut().rev() {
            if scope.get(&name).is_some() {
                scope.insert(name, value);
                return true;
            }
            if !scope.nested {
                break;
            }
        }
        false
    }

    // Create a new variable in the current scope
    // This panics if no scopes have been created
    fn create<S: Into<String>>(&mut self, name: S, value: Litteral) {
        let name = name.into();
        self.scopes.last_mut().unwrap().insert(name, value);
    }
}

/// Represents an Interpreter holding context allowing it to function
struct Interpreter<C> {
    ctx: C,
    // This allows us to implement lexical scoping
    scopes: Scopes,
    // Keeping track of functions by their name
    functions: HashMap<String, Function>,
}

impl<C: Context> Interpreter<C> {
    fn new(ctx: C) -> Self {
        Interpreter {
            ctx,
            scopes: Scopes::new(),
            functions: HashMap::new(),
        }
    }
    fn print_litteral(&mut self, l: &Litteral) {
        match l {
            Litteral::I64(i) => self.ctx.print(&format!("{}\n", i)),
            Litteral::Str(s) => {
                self.ctx.print(s);
                self.ctx.print("\n");
            }
            Litteral::Bool(s) => self.ctx.print(&format!("{}\n", s)),
        }
    }

    fn read_name(&mut self, name: &str) -> InterpreterResult<&Litteral> {
        self.scopes
            .get(name)
            .ok_or_else(|| format!("Trying to use undefined variable {}", name).into())
    }

    fn eval_block(&mut self, exprs: &[Expr]) -> InterpreterResult<Litteral> {
        let mut res = VOID;
        for e in exprs {
            res = self.eval_expr(e)?;
        }
        Ok(res)
    }

    fn eval_bin_op(&mut self, op: Op, left: &Expr, right: &Expr) -> InterpreterResult<Litteral> {
        let left = self.eval_expr(left)?;
        let right = self.eval_expr(right)?;
        match op {
            Op::Equal => Ok(Litteral::Bool(left == right)),
            Op::NotEqual => Ok(Litteral::Bool(left != right)),
            // All of these only work on ints
            Op::Leq
            | Op::Less
            | Op::Geq
            | Op::Greater
            | Op::Add
            | Op::Sub
            | Op::Mul
            | Op::Div
            | Op::Mod => {
                let (l, r) = match (left, right) {
                    (Litteral::I64(l), Litteral::I64(r)) => Ok((l, r)),
                    (l, r) => fail(format!(
                        "Op {:?} only works on I64, but got {:?} and {:?}",
                        op, l, r
                    )),
                }?;
                let res = match op {
                    Op::Leq => Litteral::Bool(l <= r),
                    Op::Less => Litteral::Bool(l < r),
                    Op::Geq => Litteral::Bool(l >= r),
                    Op::Greater => Litteral::Bool(l > r),
                    Op::Add => Litteral::I64(l + r),
                    Op::Sub => Litteral::I64(l - r),
                    Op::Mul => Litteral::I64(l * r),
                    Op::Div => Litteral::I64(l / r),
                    Op::Mod => Litteral::I64(l % r),
                    _ => unreachable!(),
                };
                Ok(res)
            }
        }
    }

    fn eval_conditional_op(
        &mut self,
        op: BoolOp,
        left: &Expr,
        right: &Expr,
    ) -> InterpreterResult<Litteral> {
        let left = match self.eval_expr(left)? {
            Litteral::Bool(b) => b,
            wrong_type => {
                return fail(format!(
                    "Expected boolean with {:?}, but found {:?}",
                    op, wrong_type
                ));
            }
        };
        let short = match op {
            BoolOp::And => false,
        };
        if left == short {
            return Ok(Litteral::Bool(short));
        };
        self.eval_expr(right)
    }

    fn eval_if_else(
        &mut self,
        cond: &Expr,
        if_part: &[Expr],
        else_part: &[Expr],
    ) -> InterpreterResult<Litteral> {
        let cond = match self.eval_expr(cond)? {
            Litteral::Bool(b) => b,
            wrong_type => {
                return fail(format!(
                    "Expected boolean in condition, but got {:?}",
                    wrong_type
                ))
            }
        };
        // Because we haven't evaluated the left and right parts, this does the right thing
        if cond {
            self.eval_block(if_part)
        } else {
            self.eval_block(else_part)
        }
    }

    fn eval_expr(&mut self, e: &Expr) -> InterpreterResult<Litteral> {
        match e {
            Expr::Call(name, args) => {
                let mut litterals: Vec<Litteral> = Vec::new();
                for a in args {
                    litterals.push(self.eval_expr(a)?);
                }
                self.call_function(name, &litterals)
            }
            Expr::Litt(l) => Ok(l.clone()),
            Expr::Name(n) => Ok(self.read_name(n)?.clone()),
            Expr::Declare(name, e) => {
                let result = self.eval_expr(e)?;
                self.scopes.create(name, result.clone());
                Ok(result)
            }
            Expr::Assign(name, e) => {
                let result = self.eval_expr(e)?;
                if self.scopes.set(name, result.clone()) {
                    Ok(result)
                } else {
                    fail(format!("Trying to assign to undeclared variable {}", name))
                }
            }
            Expr::Block(exprs) => {
                self.scopes.enter(true);
                let res = self.eval_block(exprs);
                self.scopes.exit();
                res
            }
            Expr::BinOp(op, left, right) => self.eval_bin_op(*op, left, right),
            Expr::ConditionalOp(op, left, right) => self.eval_conditional_op(*op, left, right),
            Expr::IfElse(cond, if_part, right_part) => self.eval_if_else(cond, if_part, right_part),
        }
    }

    fn call_function(&mut self, name: &str, args: &[Litteral]) -> InterpreterResult<Litteral> {
        self.scopes.enter(false);
        if name == "print" {
            let arg = args.get(0).ok_or("Not enough arguments to print")?;
            self.print_litteral(arg);
            self.scopes.exit();
            return Ok(VOID);
        };
        let res = match self.functions.get(name) {
            None => fail(format!("Trying to call undefined function {}", name)),
            Some(f) => {
                if args.len() != f.args.len() {
                    return fail(format!(
                        "Incorrect number of arguments to function {}\n.Expected {}, but got {}",
                        f.name,
                        f.args.len(),
                        args.len()
                    ));
                };
                for (i, arg) in args.iter().enumerate() {
                    self.scopes.create(f.args[i].clone(), arg.clone());
                }
                // We need to clone, because Rust doesn't know that evaluation
                // won't change the contents of f
                let body = f.body.clone();
                self.eval_block(&body)
            }
        };
        self.scopes.exit();
        res
    }

    fn interpret(&mut self, ast: &AST) -> InterpreterResult<Litteral> {
        for f in &ast.functions {
            if self.functions.insert(f.name.clone(), f.clone()).is_some() {
                return fail(format!("Redefinition of function {}", f.name));
            }
        }
        self.call_function("main", &[])
    }
}

/// Interpret a program given some context for the interpreter to use.
pub fn interpret<C: Context>(ctx: C, ast: &AST) -> InterpreterResult<Litteral> {
    let mut interpreter = Interpreter::new(ctx);
    interpreter.interpret(ast)
}
