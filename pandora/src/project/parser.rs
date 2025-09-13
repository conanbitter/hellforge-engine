use pomelo::pomelo;

pomelo! {
    %include {
        use crate::project::ast::*;
        use crate::project::tasks::ResType;
        use super::ParserState;
    }
    %token #[derive(Clone,Debug)] pub enum Token {};

    %type Int i32;
    %type Str String;
    %type Name String;
    %type int_list PropValue;
    %type value PropValue;
    %type param (String,PropValue);
    %type param_list Props;
    %type params Props;
    %type class ResType;
    %type object Node;
    %type object_name Option<String>;
    %type root Node;
    %type item Node;
    %type folder Node;
    %type package Node;
    %type item_list Vec<Node>;
    %type valobj PropValue;

    %extra_argument ParserState;

    %syntax_error {
        if let Some(sometoken) = token{
            print!("[Ln {}, Col {}] ERROR: got {}, expecting", extra.line, extra.col, sometoken);
        }else{
            print!("[Ln {}, Col {}] ERROR: expecting", extra.line, extra.col);
        }
        for extoken in expected{
            if let Some(sometoken) = extoken.token{
                print!(" {}", sometoken);
            }else{
                print!(" {}", extoken.name);
            }
        }
        println!();
        Err(())
    }

    root ::= package(pkg) { pkg };
    root ::= object(obj) { obj };

    package ::= KwPackage Name(n) LBracket item_list(il) RBracket { Node::Package(n,None,il) };
    package ::= KwPackage Name(n) params(p) LBracket item_list(il) RBracket { Node::Package(n,Some(p),il) };
    item_list ::= item_list(mut il) item(it) { il.push(it); il };
    item_list ::= item(it) { vec![it] };

    item ::= folder(fl) { fl };
    item ::= object(obj) { obj };

    folder ::= Str(s) params(p) LBracket item_list(il) RBracket { Node::Folder(s, Some(p), il) };
    folder ::= Str(s) LBracket item_list(il) RBracket { Node::Folder(s, None, il) };
    object ::= class(c) Name(n) { Node::Object(c, Some(n), None) };
    object ::= class(c) object_name(n) params(p) { Node::Object(c, n, Some(p)) };
    object ::= class(c) object_name(n) KwImport Str(s) { Node::ObjectImport(c, n, s) };
    object_name ::= { None };
    object_name ::= Name(n) { Some(n) };
    object_name ::= Asterisk { Some(String::from("*")) };

    class ::= KwTex { ResType::Texture };
    class ::= KwFont { ResType::Font };
    class ::= KwSprite { ResType::Sprite };
    class ::= KwIntMap { ResType::IntMap };
    class ::= KwExtMap { ResType::ExtMap };

    params ::= LParen param_list(pl) RParen { pl };
    param_list ::= param_list(mut list) Comma param(p) { list.push(p); list };
    param_list ::= param(p) { vec![p] };

    param ::= Name(key) { (key,PropValue::Empty) };
    param ::= Name(key) value(val) { (key, val) };
    value ::= Str(st) { PropValue::Str(st) };
    value ::= Name(n) { PropValue::Const(const_from_string(n)) };
    value ::= int_list(il) { il };
    value ::= valobj(vo) { vo };

    int_list ::= Int(v) { PropValue::Int(v) };
    int_list ::= Int(v1) Int(v2) { PropValue::Int2(v1,v2) };
    int_list ::= Int(v1) Int(v2) Int(v3) Int(v4) { PropValue::Int4(v1,v2,v3,v4) };

    valobj ::= Name(n) params(pl) { PropValue::ValObj(n, pl) };
}

pub use parser::Parser;
pub use parser::Token;

impl std::fmt::Display for parser::Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Int(_) => write!(f, "number"),
            Token::Str(_) => write!(f, "string"),
            Token::Name(_) => write!(f, "name"),
            Token::KwPackage => write!(f, "'package'"),
            Token::LBracket => write!(f, "'{{'"),
            Token::RBracket => write!(f, "'}}'"),
            Token::KwImport => write!(f, "'import'"),
            Token::Asterisk => write!(f, "'*'"),
            Token::KwTex => write!(f, "'tex'"),
            Token::KwFont => write!(f, "'font'"),
            Token::KwSprite => write!(f, "'sprite'"),
            Token::KwIntMap => write!(f, "'intmap'"),
            Token::KwExtMap => write!(f, "'extmap'"),
            Token::LParen => write!(f, "'('"),
            Token::RParen => write!(f, "')'"),
            Token::Comma => write!(f, "','"),
        }
    }
}

pub struct ParserState {
    pub line: u32,
    pub col: u32,
}
