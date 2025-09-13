use crate::project::{
    lexer::Lexer,
    parser::{Parser, ParserState},
    tasks::{PackageTask, generate_package},
};

pub mod ast;
mod lexer;
pub mod parser;
mod tasks;

pub fn tasks_from_string(source: &str) -> anyhow::Result<PackageTask> {
    let mut par = Parser::new(ParserState { line: 1, col: 1 });
    let mut lex = Lexer::new(source);

    while let Some((token, line, col)) = lex.next() {
        //println!("===={:?}", token);
        par.extra_mut().line = line;
        par.extra_mut().col = col;
        par.parse(token).unwrap();
    }
    let tree = par.end_of_input().unwrap().0;
    let package = generate_package(&tree)?;
    Ok(package)
}
