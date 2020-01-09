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
const PROG_8: &'static str = include_str!("../test-programs/8.iku");
const PROG_9: &'static str = include_str!("../test-programs/9.iku");
const PROG_10: &'static str = include_str!("../test-programs/10.iku");
const PROG_11: &'static str = include_str!("../test-programs/11.iku");
const PROG_12: &'static str = include_str!("../test-programs/12.iku");
const PROG_13: &'static str = include_str!("../test-programs/13.iku");
const PROG_14: &'static str = include_str!("../test-programs/14.iku");

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
    let body = vec![Expr::Call(
        String::from("print"),
        vec![Expr::Litt(Litteral::I64(2))],
    )];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "2\n");
}

#[test]
fn test_prog_2() {
    let lexer = Lexer::new(PROG_2);
    let res = ASTParser::new().parse(lexer);
    let body = vec![Expr::Call(
        String::from("print"),
        vec![Expr::Litt(Litteral::I64(-2))],
    )];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
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
    let body = vec![Expr::Call(
        String::from("print"),
        vec![Expr::Litt(Litteral::Str(litt))],
    )];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, &format!("{}\n", PROG_3_LITT));
}

#[test]
fn test_prog_4() {
    let lexer = Lexer::new(PROG_4);
    let res = ASTParser::new().parse(lexer);
    let body = vec![
        Expr::Call(String::from("print"), vec![Expr::Litt(Litteral::I64(1))]),
        Expr::Call(String::from("print"), vec![Expr::Litt(Litteral::I64(2))]),
    ];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "1\n2\n");
}

#[test]
fn test_prog_5() {
    let lexer = Lexer::new(PROG_5);
    let res = ASTParser::new().parse(lexer);
    let body = vec![
        Expr::Call(String::from("print"), vec![Expr::Litt(Litteral::I64(1))]),
        Expr::Call(String::from("print"), vec![Expr::Litt(Litteral::I64(2))]),
    ];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "1\n2\n");
}

#[test]
fn test_prog_6() {
    let lexer = Lexer::new(PROG_6);
    let res = ASTParser::new().parse(lexer);
    let body = vec![
        Expr::Call(String::from("print"), vec![Expr::Litt(Litteral::I64(6))]),
        Expr::Call(String::from("print"), vec![Expr::Litt(Litteral::I64(6))]),
    ];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "6\n6\n");
}

#[test]
fn test_prog_7() {
    let lexer = Lexer::new(PROG_7);
    let res = ASTParser::new().parse(lexer);
    let body = vec![
        Expr::Declare("x".into(), Box::new(Expr::Litt(Litteral::I64(2)))),
        Expr::Declare(
            "y".into(),
            Box::new(Expr::Declare(
                "z".into(),
                Box::new(Expr::Litt(Litteral::I64(2))),
            )),
        ),
        Expr::Call("print".into(), vec![Expr::Name("x".into())]),
        Expr::Call("print".into(), vec![Expr::Name("y".into())]),
        Expr::Call("print".into(), vec![Expr::Name("z".into())]),
    ];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "2\n2\n2\n");
}

#[test]
fn test_prog_8() {
    let lexer = Lexer::new(PROG_8);
    let res = ASTParser::new().parse(lexer);
    let body = vec![
        Expr::Declare("x".into(), Box::new(Expr::Litt(Litteral::I64(2)))),
        Expr::Call("print".into(), vec![Expr::Name("x".into())]),
        Expr::Assign("x".into(), Box::new(Expr::Litt(Litteral::I64(3)))),
        Expr::Call("print".into(), vec![Expr::Name("x".into())]),
    ];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "2\n3\n");
}

#[test]
fn test_prog_9() {
    let lexer = Lexer::new(PROG_9);
    let res = ASTParser::new().parse(lexer);
    let body_foo = vec![Expr::Call(
        "print".into(),
        vec![Expr::Litt(Litteral::I64(1))],
    )];
    let body_main = vec![Expr::Call("foo".into(), vec![])];
    let ast = AST {
        functions: vec![
            Function {
                name: "foo".into(),
                args: vec![],
                body: body_foo,
            },
            Function {
                name: "main".into(),
                args: vec![],
                body: body_main,
            },
        ],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "1\n");
}

#[test]
fn test_prog_10() {
    let lexer = Lexer::new(PROG_10);
    let res = ASTParser::new().parse(lexer);
    let body_foo = vec![
        Expr::Call("print".into(), vec![Expr::Name("x".into())]),
        Expr::Call("print".into(), vec![Expr::Name("y".into())]),
    ];
    let body_main = vec![Expr::Call(
        "foo".into(),
        vec![Expr::Litt(Litteral::I64(1)), Expr::Litt(Litteral::I64(2))],
    )];
    let ast = AST {
        functions: vec![
            Function {
                name: "foo".into(),
                args: vec!["x".into(), "y".into()],
                body: body_foo,
            },
            Function {
                name: "main".into(),
                args: vec![],
                body: body_main,
            },
        ],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "1\n2\n");
}

#[test]
fn test_prog_11() {
    let lexer = Lexer::new(PROG_11);
    let res = ASTParser::new().parse(lexer);
    let body = vec![
        Expr::Call("print".into(), vec![Expr::Litt(Litteral::Bool(true))]),
        Expr::Call("print".into(), vec![Expr::Litt(Litteral::Bool(false))]),
    ];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "true\nfalse\n");
}

#[test]
fn test_prog_12() {
    let lexer = Lexer::new(PROG_12);
    let res = ASTParser::new().parse(lexer);
    let body = vec![
        Expr::Declare("x".into(), Box::new(Expr::Litt(Litteral::I64(2)))),
        Expr::Declare(
            "y".into(),
            Box::new(Expr::Block(vec![
                Expr::Declare("x".into(), Box::new(Expr::Litt(Litteral::I64(3)))),
                Expr::Name("x".into()),
            ])),
        ),
        Expr::Call("print".into(), vec![Expr::Name("x".into())]),
        Expr::Call("print".into(), vec![Expr::Name("y".into())]),
    ];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "2\n3\n");
}

#[test]
fn test_prog_13() {
    let lexer = Lexer::new(PROG_13);
    let res = ASTParser::new().parse(lexer);
    let body = vec![
        Expr::Call(
            "print".into(),
            vec![Expr::BinOp(
                Op::Equal,
                Box::new(Expr::Litt(Litteral::I64(1))),
                Box::new(Expr::Litt(Litteral::I64(1))),
            )],
        ),
        Expr::Call(
            "print".into(),
            vec![Expr::BinOp(
                Op::Equal,
                Box::new(Expr::Litt(Litteral::I64(1))),
                Box::new(Expr::Litt(Litteral::I64(2))),
            )],
        ),
    ];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "true\nfalse\n");
}

#[test]
fn test_prog_14() {
    let lexer = Lexer::new(PROG_14);
    let res = ASTParser::new().parse(lexer);
    let body = vec![
        Expr::Call(
            "print".into(),
            vec![Expr::BinOp(
                Op::Greater,
                Box::new(Expr::Litt(Litteral::I64(1))),
                Box::new(Expr::Litt(Litteral::I64(1))),
            )],
        ),
        Expr::Call(
            "print".into(),
            vec![Expr::BinOp(
                Op::Geq,
                Box::new(Expr::Litt(Litteral::I64(1))),
                Box::new(Expr::Litt(Litteral::I64(1))),
            )],
        ),
        Expr::Call(
            "print".into(),
            vec![Expr::BinOp(
                Op::Less,
                Box::new(Expr::Litt(Litteral::I64(1))),
                Box::new(Expr::Litt(Litteral::I64(1))),
            )],
        ),
        Expr::Call(
            "print".into(),
            vec![Expr::BinOp(
                Op::Leq,
                Box::new(Expr::Litt(Litteral::I64(1))),
                Box::new(Expr::Litt(Litteral::I64(1))),
            )],
        ),
    ];
    let ast = AST {
        functions: vec![Function {
            name: "main".into(),
            args: vec![],
            body,
        }],
    };
    assert_eq!(res.as_ref(), Ok(&ast));
    let mut interpreted = String::new();
    assert!(interpret(FakeContext::new(&mut interpreted), &ast).is_ok());
    assert_eq!(&interpreted, "false\ntrue\nfalse\ntrue\n");
}
