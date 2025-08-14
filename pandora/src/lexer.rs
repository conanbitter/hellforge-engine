use crate::parser::Token;
pub struct Lexer {
    data: &'static str,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &'static str) -> Lexer {
        Lexer { data: source, pos: 0 }
    }
}
