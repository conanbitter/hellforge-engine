use std::str::Chars;

use crate::parser::Token;
pub struct Lexer<'a> {
    data: Chars<'a>,
    cur_char: char,
    eof: bool,
    line: u32,
    col: u32,
}

enum NameVariant {
    Name(String),
    Keyword(Token),
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        let mut result = Lexer {
            data: source.chars(),
            cur_char: '\0',
            eof: false,
            line: 1,
            col: 1,
        };
        result.forward();
        result
    }

    pub fn next(&mut self) -> Option<(Token, u32, u32)> {
        self.skip_spaces();
        if self.eof {
            return None;
        }
        while self.cur_char == '#' {
            self.skip_comments();
            self.skip_spaces();
            if self.eof {
                return None;
            }
        }

        let line = self.line;
        let col = self.col - 1;

        match self.cur_char {
            '0'..='9' => Some((Token::Int(self.read_int()), line, col)),
            '"' => Some((Token::Str(self.read_str()), line, col)),
            '(' => {
                self.forward();
                Some((Token::LParen, line, col))
            }
            ')' => {
                self.forward();
                Some((Token::RParen, line, col))
            }
            '{' => {
                self.forward();
                Some((Token::LBracket, line, col))
            }
            '}' => {
                self.forward();
                Some((Token::RBracket, line, col))
            }
            ',' => {
                self.forward();
                Some((Token::Comma, line, col))
            }
            '*' => {
                self.forward();
                Some((Token::Asterisk, line, col))
            }
            _ => {
                if self.cur_char.is_ascii_alphabetic() {
                    match self.read_name() {
                        NameVariant::Keyword(kw) => Some((kw, line, col)),
                        NameVariant::Name(name) => Some((Token::Name(name), line, col)),
                    }
                } else {
                    None
                }
            }
        }
    }

    fn forward(&mut self) {
        if let Some(next_char) = self.data.next() {
            self.cur_char = next_char;
            if next_char == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        } else {
            self.eof = true;
        }
    }

    fn skip_spaces(&mut self) {
        while !self.eof && self.cur_char.is_whitespace() {
            self.forward();
        }
    }

    fn skip_comments(&mut self) {
        while !self.eof && self.cur_char != '\n' {
            self.forward();
        }
        self.forward();
    }

    fn read_int(&mut self) -> i32 {
        let mut result = String::new();
        while !self.eof && self.cur_char.is_ascii_digit() {
            result.push(self.cur_char);
            self.forward();
        }
        result.parse().unwrap_or(0)
    }

    fn read_str(&mut self) -> String {
        self.forward();
        let mut escaping = false;
        let mut result = String::new();
        while !self.eof {
            if escaping {
                result.push(self.cur_char);
                self.forward();
                escaping = false;
            } else {
                match self.cur_char {
                    '"' => break,
                    '\\' => escaping = true,
                    _ => result.push(self.cur_char),
                }
                self.forward();
            }
        }
        self.forward();
        result
    }

    fn read_name(&mut self) -> NameVariant {
        let mut result = String::new();
        while !self.eof && (self.cur_char.is_ascii_alphanumeric() || self.cur_char == '_') {
            result.push(self.cur_char);
            self.forward();
        }
        match result.to_lowercase().as_str() {
            "package" => NameVariant::Keyword(Token::KwPackage),
            "tex" => NameVariant::Keyword(Token::KwTex),
            "font" => NameVariant::Keyword(Token::KwFont),
            "sprite" => NameVariant::Keyword(Token::KwSprite),
            "import" => NameVariant::Keyword(Token::KwImport),
            "intmap" => NameVariant::Keyword(Token::KwIntMap),
            "extmap" => NameVariant::Keyword(Token::KwExtMap),
            _ => NameVariant::Name(result),
        }
    }
}
