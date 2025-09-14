use std::{fs, path::Path};

use crate::project::{
    ast::Node,
    lexer::Lexer,
    parser::{Parser, ParserState},
    tasks::{PackageTask, Task, generate_project, generate_task},
};

pub mod ast;
mod lexer;
pub mod parser;
mod tasks;

fn parse_file<P: AsRef<Path>>(source_file: P) -> anyhow::Result<Node> {
    let source = fs::read_to_string(source_file)?;

    let mut par = Parser::new(ParserState { line: 1, col: 1 });
    let mut lex = Lexer::new(&source);

    while let Some((token, line, col)) = lex.next() {
        par.extra_mut().line = line;
        par.extra_mut().col = col;
        par.parse(token).unwrap();
    }
    Ok(par.end_of_input().unwrap().0)
}

pub fn project_from_file<P: AsRef<Path>>(source_file: P) -> anyhow::Result<PackageTask> {
    let tree = parse_file(source_file)?;
    generate_project(&tree)
}

pub fn task_from_file<P: AsRef<Path>>(source_file: P) -> anyhow::Result<Task> {
    let tree = parse_file(source_file)?;
    generate_task(&tree)
}
