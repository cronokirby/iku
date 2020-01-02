/// Represents an expression in the Iku language.
///
/// Expressions can be evaluated to some kind of value.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Print(Box<Expr>),
    I32(i32),
    Str(String),
}

/// Represents the full abstract syntax tree of an Iku program.
///
/// At the moment, a program is synonymous with a single file. This will
/// probably change at some point.
#[derive(Clone, Debug, PartialEq)]
pub enum AST {
    FuncMain(Expr),
}

/// Handle escape sequences when processing a litteral string.
#[allow(dead_code)]
pub fn process_string_litteral(input: &str) -> String {
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
    acc
}
