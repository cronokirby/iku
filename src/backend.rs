use crate::ast::*;
use std::io;

fn expr(buf: &mut impl io::Write, e: Expr) -> io::Result<()> {
    match e {
        Expr::Print(e) => {
            write!(buf, "println(")?;
            expr(buf, *e)?;
            write!(buf, ")")
        }
        Expr::I32(i) => write!(buf, "{:?}", i),
        Expr::Str(s) => write!(buf, "{:?}", s),
    }
}

pub fn generate(buf: &mut impl io::Write, ast: AST) -> io::Result<()> {
    match ast {
        AST::FuncMain(e) => {
            writeln!(buf, "package main")?;
            writeln!(buf, "func main() {{")?;
            expr(buf, e)?;
            writeln!(buf, "\n}}")
        }
    }
}
