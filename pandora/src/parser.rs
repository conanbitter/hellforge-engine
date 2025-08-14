use pomelo::pomelo;

pomelo! {
    %token #[derive(Clone,Debug)] pub enum Token {};

    %type Int i32;
    %type Str String;
    %type Name String;

    package ::= Int;
    package ::= Str;
    package ::= Name;
    package ::= LParen;
    package ::= RParen;
    package ::= LBracket;
    package ::= RBracket;
    package ::= Comma;
    package ::= Asterisk;
    package ::= KwPackage;
    package ::= KwTex;
    package ::= KwFont;
    package ::= KwSprite;
    package ::= KwImport;
    package ::= KwIntMap;
    package ::= KwExtMap;

}

pub use parser::Token;
