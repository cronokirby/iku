#[derive(Debug, PartialEq)]
pub enum Expr {
    Print(Box<Expr>),
    I32(i32),
}

#[derive(Debug, PartialEq)]
pub enum AST {
    FuncMain(Expr),
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
}
