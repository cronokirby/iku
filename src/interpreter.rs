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
                let mut res = VOID;
                // We need to clone, because Rust doesn't know that evaluation
                // won't change the contents of f
                for e in f.body.clone() {
                    res = self.eval_expr(&e)?;
                }
                Ok(res)
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
