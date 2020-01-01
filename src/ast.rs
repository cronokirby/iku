#[derive(Debug, PartialEq)]
pub enum Expr {
    Print(Box<Expr>),
    I32(i32),
    Str(String),
}

#[derive(Debug, PartialEq)]
pub enum AST {
    FuncMain(Expr),
}

/// Handle escape sequences
#[allow(dead_code)]
pub fn process_string_litteral(input: &str) -> String {
    println!("{:?}", input);
    let mut acc = String::new();
    let mut escaping = false;
    for c in input.chars() {
        let was_escaping = escaping;
        match (escaping, c) {
            // This will only be the first and last
            (_, '"') => {}
            (true, 'n') => acc.push_str("\n"),
            (true, '\\') => acc.push('\\'),
            (true, 'r') => acc.push('\r'),
            (true, 't') => acc.push('\t'),
            (true, '0') => acc.push('\0'),
            (true, c) => {
                acc.push('\\');
                acc.push(c)
            }
            (false, '\\') => escaping = true,
            (false, c) => acc.push(c),
        }
        if was_escaping {
            escaping = false;
        }
    }
    println!("{:?}", acc);
    acc
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_ast::ASTParser;

    #[test]
    fn test_1() {
        let prog = include_str!("../test-programs/1.iku");
        let res = ASTParser::new().parse(prog);
        let ast = AST::FuncMain(Expr::Print(Box::new(Expr::I32(2))));
        assert_eq!(res, Ok(ast));
    }

    #[test]
    fn test_2() {
        let prog = include_str!("../test-programs/2.iku");
        let res = ASTParser::new().parse(prog);
        let ast = AST::FuncMain(Expr::Print(Box::new(Expr::I32(-2))));
        assert_eq!(res, Ok(ast));
    }

    #[test]
    fn test_3() {
        let prog = include_str!("../test-programs/3.iku");
        let res = ASTParser::new().parse(prog);
        let litt = String::from("\n\t\0\r\\今日はhello");
        let ast = AST::FuncMain(Expr::Print(Box::new(Expr::Str(litt))));
        assert_eq!(res, Ok(ast));
    }
}
