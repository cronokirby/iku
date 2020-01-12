use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parse_ast);
mod ast;
mod interpreter;
mod lexer;
mod scopes;
#[cfg(test)]
mod test_programs;
mod typer;

#[derive(Debug, StructOpt)]
#[structopt(name = "iku", about = "The iku programming language")]
struct Opt {
    /// A file containing a program in iku
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let mut prog_file = File::open(opt.file)?;
    let mut prog = String::new();
    prog_file.read_to_string(&mut prog)?;
    let lexer = lexer::Lexer::new(&prog);
    let ast = parse_ast::ASTParser::new().parse(lexer).unwrap();
    if let Err(e) = interpreter::interpret(interpreter::RealContext, &ast) {
        println!("Interpreter Error: {:?}", e);
    };
    Ok(())
}
