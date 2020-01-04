use crate::ast::*;
use crate::interpreter::{interpret, Context};
use crate::lexer::Lexer;
use crate::parse_ast::ASTParser;

const PROG_1: &'static str = include_str!("../test-programs/1.iku");
const PROG_2: &'static str = include_str!("../test-programs/2.iku");
const PROG_3: &'static str = include_str!("../test-programs/3.iku");
const PROG_4: &'static str = include_str!("../test-programs/4.iku");
const PROG_5: &'static str = include_str!("../test-programs/5.iku");
const PROG_6: &'static str = include_str!("../test-programs/6.iku");
const PROG_7: &'static str = include_str!("../test-programs/7.iku");

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
    let lexer = Lexer::new(PROG_1);
    let res = ASTParser::new().parse(lexer);
    let ast = AST::FuncMain(vec![Expr::Call(
        String::from("print"),
        Box::new(Expr::Litt(Litteral::I64(2))),
    )]);
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "2\n");
}

#[test]
fn test_prog_2() {
    let lexer = Lexer::new(PROG_2);
    let res = ASTParser::new().parse(lexer);
    let ast = AST::FuncMain(vec![Expr::Call(
        String::from("print"),
        Box::new(Expr::Litt(Litteral::I64(-2))),
    )]);
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "-2\n");
}

const PROG_3_LITT: &'static str = "\n\t\r\\今日はhello";

#[test]
fn test_prog_3() {
    let lexer = Lexer::new(PROG_3);
    let res = ASTParser::new().parse(lexer);
    let litt = String::from(PROG_3_LITT);
    let ast = AST::FuncMain(vec![Expr::Call(
        String::from("print"),
        Box::new(Expr::Litt(Litteral::Str(litt))),
    )]);
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, &format!("{}\n", PROG_3_LITT));
}

#[test]
fn test_prog_4() {
    let lexer = Lexer::new(PROG_4);
    let res = ASTParser::new().parse(lexer);
    let ast = AST::FuncMain(vec![
        Expr::Call(
            String::from("print"),
            Box::new(Expr::Litt(Litteral::I64(1))),
        ),
        Expr::Call(
            String::from("print"),
            Box::new(Expr::Litt(Litteral::I64(2))),
        ),
    ]);
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "1\n2\n");
}

#[test]
fn test_prog_5() {
    let lexer = Lexer::new(PROG_5);
    let res = ASTParser::new().parse(lexer);
    let ast = AST::FuncMain(vec![
        Expr::Call(
            String::from("print"),
            Box::new(Expr::Litt(Litteral::I64(1))),
        ),
        Expr::Call(
            String::from("print"),
            Box::new(Expr::Litt(Litteral::I64(2))),
        ),
    ]);
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "1\n2\n");
}

#[test]
fn test_prog_6() {
    let lexer = Lexer::new(PROG_6);
    let res = ASTParser::new().parse(lexer);
    let ast = AST::FuncMain(vec![
        Expr::Call(
            String::from("print"),
            Box::new(Expr::Litt(Litteral::I64(6))),
        ),
        Expr::Call(
            String::from("print"),
            Box::new(Expr::Litt(Litteral::I64(6))),
        ),
    ]);
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "6\n6\n");
}

#[test]
fn test_prog_7() {
    let lexer = Lexer::new(PROG_7);
    let res = ASTParser::new().parse(lexer);
    let ast = AST::FuncMain(vec![
        Expr::Declare("x".into(), Box::new(Expr::Litt(Litteral::I64(2)))),
        Expr::Declare(
            "y".into(),
            Box::new(Expr::Declare(
                "z".into(),
                Box::new(Expr::Litt(Litteral::I64(2))),
            )),
        ),
        Expr::Call("print".into(), Box::new(Expr::Name("x".into()))),
        Expr::Call("print".into(), Box::new(Expr::Name("y".into()))),
        Expr::Call("print".into(), Box::new(Expr::Name("z".into()))),
    ]);
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "2\n2\n2\n");
}
