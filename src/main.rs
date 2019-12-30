mod lexer;

fn main() {
    let program = include_str!("../test-programs/1.iku");
    for t in lexer::lex(program.chars()) {
        println!("{:?}", t);
    }
}
