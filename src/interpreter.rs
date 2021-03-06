use crate::ast::*;
use std::collections::HashMap;
use crate::scopes::Scopes;

// Unit is used like void in other languages.
fn unit() -> Litteral {
    Litteral::Tuple(vec![])
}

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

/// Represents an Interpreter holding context allowing it to function
struct Interpreter<C> {
    ctx: C,
    // This allows us to implement lexical scoping
    scopes: Scopes<Litteral>,
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
        self.ctx.print(&format!("{}\n", l));
    }

    fn read_name(&mut self, name: &str) -> InterpreterResult<&Litteral> {
        self.scopes
            .get(name)
            .ok_or_else(|| format!("Trying to use undefined variable {}", name).into())
    }

    fn eval_block(&mut self, exprs: &[Expr]) -> InterpreterResult<Litteral> {
        let mut res = unit();
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
            BoolOp::Or => true,
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
            Expr::Not(expr) => match self.eval_expr(expr)? {
                Litteral::Bool(b) => Ok(Litteral::Bool(!b)),
                wrong_type => fail(format!(
                    "The operator ! only applies to Bool, but got {:?}",
                    wrong_type
                )),
            },
            Expr::MakeTuple(exprs) => {
                let mut litterals = Vec::new();
                for e in exprs {
                    litterals.push(self.eval_expr(e)?);
                }
                Ok(Litteral::Tuple(litterals))
            }
        }
    }

    fn call_function(&mut self, name: &str, args: &[Litteral]) -> InterpreterResult<Litteral> {
        self.scopes.enter(false);
        if name == "print" {
            let arg = args.get(0).ok_or("Not enough arguments to print")?;
            self.print_litteral(arg);
            self.scopes.exit();
            return Ok(unit());
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
                for (i, (arg, _)) in f.args.iter().enumerate() {
                    self.scopes.create(arg.clone(), args[i].clone());
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
