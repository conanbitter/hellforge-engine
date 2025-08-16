use pomelo::pomelo;

pomelo! {
    %include {
        use crate::ast::*;
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
    %type class ObjectType;
    %type object Node;
    %type object_name Option<String>;
    %type root Node;
    %type item Node;
    %type folder Node;
    %type package Node;
    %type item_list Vec<Node>;

    %syntax_error {
        println!("[Error] Got {:?}, expecting:", token);
        for extoken in expected{
            println!("{}",extoken.name)
        }
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

    class ::= KwTex { ObjectType::Texture };
    class ::= KwFont { ObjectType::Font };
    class ::= KwSprite { ObjectType::Sprite };
    class ::= KwIntMap { ObjectType::IntMap };
    class ::= KwExtMap { ObjectType::ExtMap };

    params ::= LParen param_list(pl) RParen { pl };
    param_list ::= param_list(mut list) Comma param(p) { list.push(p); list };
    param_list ::= param(p) { vec![p] };

    param ::= Name(key) { (key,PropValue::Empty) };
    param ::= Name(key) value(val) { (key, val) };
    value ::= Str(st) { PropValue::Str(st) };
    value ::= Name(n) { PropValue::Const(const_from_string(n)) };
    value ::= int_list(il) { il };
    value ::= valobj { println!("value ::= valobj"); PropValue::Empty };

    int_list ::= Int(v) { PropValue::Int(v) };
    int_list ::= Int(v1) Int(v2) { PropValue::Int2(v1,v2) };
    int_list ::= Int(v1) Int(v2) Int(v3) Int(v4) { PropValue::Int4(v1,v2,v3,v4) };

    valobj ::= Name params { println!("valobj ::= Name params"); };

}

pub use parser::Parser;
pub use parser::Token;
