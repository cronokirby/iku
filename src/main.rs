#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parse_ast);
mod ast;
mod interpreter;
#[cfg(test)]
mod test_programs;

fn main() {
    let prog = include_str!("../test-programs/1.iku");
    let ast = parse_ast::ASTParser::new().parse(prog).unwrap();
    interpreter::interpret(interpreter::RealContext, &ast);
}
