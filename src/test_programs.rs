use crate::ast::*;
use crate::interpreter::{interpret, Context};
use crate::parse_ast::ASTParser;

const PROG_1: &'static str = include_str!("../test-programs/1.iku");
const PROG_2: &'static str = include_str!("../test-programs/2.iku");
const PROG_3: &'static str = include_str!("../test-programs/3.iku");

#[derive(Debug)]
struct FakeContext<'a> {
    prints: &'a mut String,
}

impl<'a> FakeContext<'a> {
    fn new(buf: &'a mut String) -> Self {
        FakeContext { prints: buf }
    }
}

impl<'a> Context for FakeContext<'a> {
    fn print(&mut self, data: &str) {
        self.prints.push_str(data);
    }
}

#[test]
fn test_prog_1() {
    let res = ASTParser::new().parse(PROG_1);
    let ast = AST::FuncMain(Expr::Print(Box::new(Expr::I32(2))));
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    interpret(FakeContext::new(&mut interpreted), &ast);
    assert_eq!(&interpreted, "2\n");
}

#[test]
fn test_prog_2() {
    let res = ASTParser::new().parse(PROG_2);
    let ast = AST::FuncMain(Expr::Print(Box::new(Expr::I32(-2))));
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    interpret(FakeContext::new(&mut interpreted), &ast);
    assert_eq!(&interpreted, "-2\n");
}

const PROG_3_LITT: &'static str = "\n\t\r\\今日はhello";

#[test]
fn test_prog_3() {
    let res = ASTParser::new().parse(PROG_3);
    let litt = String::from(PROG_3_LITT);
    let ast = AST::FuncMain(Expr::Print(Box::new(Expr::Str(litt))));
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    interpret(FakeContext::new(&mut interpreted), &ast);
    assert_eq!(&interpreted, &format!("{}\n", PROG_3_LITT));
}
