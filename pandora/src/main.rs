use std::fs;

use crate::{
    lexer::Lexer,
    parser::{Parser, ParserState},
    tasks::generate_package,
};

mod ast;
mod image;
mod lexer;
mod parser;
mod tasks;

fn main() -> anyhow::Result<()> {
    let project_source = fs::read_to_string("test.pnd")?;

    //let project_source = String::from("123 453 \"Hel\\\\lo !\"  sd (tex) 23");

    let mut par = Parser::new(ParserState { line: 1, col: 1 });
    let mut lex = Lexer::new(&project_source);

    while let Some((token, line, col)) = lex.next() {
        //println!("===={:?}", token);
        par.extra_mut().line = line;
        par.extra_mut().col = col;
        par.parse(token).unwrap();
    }
    let tree = par.end_of_input().unwrap().0;
    let package = generate_package(&tree)?;

    println!("Package: {}", package.filename);
    println!("Tasks:");
    for task in package.tasks {
        println!("    {:?}", task);
    }

    Ok(())
}
