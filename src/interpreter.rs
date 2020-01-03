use crate::ast::*;

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
}

impl<C: Context> Interpreter<C> {
    fn print_expr(&mut self, e: &Expr) {
        match e {
            Expr::I32(i) => self.ctx.print(&format!("{}\n", i)),
            Expr::Str(s) => {
                self.ctx.print(s);
                self.ctx.print("\n");
            }
            Expr::Call(_, _) => panic!("Trying to show unreduced expression!"),
        }
    }

    fn eval_expr(&mut self, e: &Expr) -> InterpreterResult<Expr> {
        match e {
            Expr::Call(name, e) => {
                let inside = self.eval_expr(e)?;
                match name.as_ref() {
                    "print" => {
                        self.print_expr(&inside);
                        Ok(inside)
                    }
                    _ => fail(format!("Trying to call unknown function {}", name)),
                }
            }
            other => Ok(other.clone()),
        }
    }

    fn interpret(&mut self, ast: &AST) -> InterpreterResult<Expr> {
        match ast {
            AST::FuncMain(exprs) => {
                let mut res = Expr::I32(0);
                for e in exprs {
                    res = self.eval_expr(e)?;
                }
                Ok(res)
            }
        }
    }
}

/// Interpret a program given some context for the interpreter to use.
pub fn interpret<C: Context>(ctx: C, ast: &AST) -> InterpreterResult<Expr> {
    let mut interpreter = Interpreter { ctx };
    interpreter.interpret(ast)
}
