use pomelo::pomelo;

pomelo! {
    %type INT i32;
    %type STR String;
    %type NAME String;

    package ::= INT;
    package ::= STR;
    package ::= NAME;
    package ::= LPAREN;
    package ::= RPAREN;
    package ::= LBRACKET;
    package ::= RBRACKET;
    package ::= COMMA;
    package ::= KW_PACKAGE;
    package ::= KW_TEX;
    package ::= KW_FONT;
    package ::= KW_SPRITE;
    package ::= KW_IMPORT;
    package ::= KW_INTLEVEL;
    package ::= KW_EXTLEVEL;

}

pub use parser::Token;
