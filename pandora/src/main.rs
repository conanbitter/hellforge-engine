use std::fs;

use crate::{lexer::Lexer, parser::Parser};

mod ast;
mod lexer;
mod parser;

fn main() -> anyhow::Result<()> {
    let project_source = fs::read_to_string("test.pnd")?;

    //let project_source = String::from("123 453 \"Hel\\\\lo !\"  sd (tex) 23");

    let mut lex = Lexer::new(&project_source);
    let mut par = Parser::new();

    while let Some(token) = lex.next() {
        //println!("===={:?}", token);
        par.parse(token).unwrap();
    }
    let tree = par.end_of_input().unwrap();

    println!("Tree: {:?}", tree);

    Ok(())
}
