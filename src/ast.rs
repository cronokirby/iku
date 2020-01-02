#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Print(Box<Expr>),
    I32(i32),
    Str(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AST {
    FuncMain(Expr),
}

/// Handle escape sequences
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
