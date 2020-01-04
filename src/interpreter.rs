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

/// A wrapper type for results that fail in an interpreter
pub type InterpreterResult<T> = Result<T, InterpreterError>;

fn fail<T, S: Into<String>>(message: S) -> InterpreterResult<T> {
    Err(InterpreterError {
        message: message.into(),
    })
}

/// Represents an Interpreter holding context allowing it to function
struct Interpreter<C> {
    ctx: C,
    // A collection of variables defined in the main function
    vars: HashMap<String, Litteral>,
    // Keeping track of functions by their name
    functions: HashMap<String, Function>,
}

impl<C: Context> Interpreter<C> {
    fn new(ctx: C) -> Self {
        Interpreter {
            ctx,
            vars: HashMap::new(),
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
        }
    }

    fn read_name(&mut self, name: &str) -> InterpreterResult<&Litteral> {
        self.vars
            .get(name)
            .ok_or(format!("Trying to use undefined variable {}", name).into())
    }

    fn eval_expr(&mut self, e: &Expr) -> InterpreterResult<Litteral> {
        match e {
            Expr::Call(name, e) => {
                let inside = self.eval_expr(e)?;
                match name.as_ref() {
                    "print" => {
                        self.print_litteral(&inside);
                        Ok(inside)
                    }
                    _ => fail(format!("Trying to call unknown function {}", name)),
                }
            }
            Expr::Litt(l) => Ok(l.clone()),
            Expr::Name(n) => Ok(self.read_name(n)?.clone()),
            Expr::Declare(name, e) => {
                let result = self.eval_expr(e)?;
                self.vars.insert(name.clone(), result.clone());
                Ok(result)
            }
            Expr::Assign(name, e) => {
                let result = self.eval_expr(e)?;
                if self.vars.insert(name.clone(), result.clone()).is_none() {
                    fail(format!("Trying to assign to undeclared variable {}", name))
                } else {
                    Ok(result)
                }
            }
        }
    }

    fn call_function(&mut self, name: &str, arg: &Litteral) -> InterpreterResult<Litteral> {
        match name {
            "print" => {
                self.print_litteral(arg);
                return Ok(VOID);
            }
            _ => {}
        };
        match self.functions.get(name) {
            None => fail(format!("Trying to call undefined function {}", name)),
            Some(f) => {
                let mut res = VOID;
                // We need to clone, because Rust doesn't know that evaluation
                // won't change the contents of f
                for e in f.body.clone() {
                    res = self.eval_expr(&e)?;
                }
                Ok(res)
            }
        }
    }

    fn interpret(&mut self, ast: &AST) -> InterpreterResult<Litteral> {
        for f in &ast.functions {
            if self.functions.insert(f.name.clone(), f.clone()).is_some() {
                return fail(format!("Redefinition of function {}", f.name));
            }
        }
        dbg!(&self.functions);
        self.call_function("main", &VOID)
    }
}

/// Interpret a program given some context for the interpreter to use.
pub fn interpret<C: Context>(ctx: C, ast: &AST) -> InterpreterResult<Litteral> {
    let mut interpreter = Interpreter::new(ctx);
    interpreter.interpret(ast)
}
