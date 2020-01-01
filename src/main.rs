#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parse_ast);
mod ast;
mod backend;

fn main() -> std::io::Result<()> {
    let prog = include_str!("../test-programs/3.iku");
    let ast = parse_ast::ASTParser::new().parse(prog).unwrap();
    let mut out = std::io::stdout();
    backend::generate(&mut out, ast)
}
