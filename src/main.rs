#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parse_ast);
mod ast;

fn main() {
    let program = include_str!("../test-programs/1.iku");
    let res = parse_ast::ASTParser::new().parse(program);
    println!("{:?}", res);
}
