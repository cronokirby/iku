#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parse_ast);
mod ast;
mod interpreter;
mod lexer;
#[cfg(test)]
mod test_programs;

fn main() {
    let prog = include_str!("../test-programs/1.iku");
    let lexer = lexer::Lexer::new(prog);
    let ast = parse_ast::ASTParser::new().parse(lexer).unwrap();
    match interpreter::interpret(interpreter::RealContext, &ast) {
        Err(e) => println!("Interpreter Error: {:?}", e),
        _ => {}
    }
}
