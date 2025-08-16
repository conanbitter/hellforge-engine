use pomelo::pomelo;

pomelo! {
    %token #[derive(Clone,Debug)] pub enum Token {};

    %type Int i32;
    %type Str String;
    %type Name String;

    %syntax_error {
        println!("[Error] Got {:?}, expecting:", token);
        for extoken in expected{
            println!("{}",extoken.name)
        }
        Err(())
    }

    root ::= package { println!("root ::= package"); };
    root ::= object { println!("root ::= object"); };

    package ::= KwPackage Name LBracket item_list RBracket { println!("package ::= KwPackage Name {{ item_list }}"); };
    item_list ::= item_list item { println!("item_list ::= item_list item"); };
    item_list ::= item { println!("item_list ::= item"); };

    item ::= folder { println!("item ::= folder"); };
    item ::= object { println!("item ::= object"); };

    folder ::= Str(B) params LBracket item_list RBracket { println!("folder ::= Str({}) params {{ item_list }}",B); };
    folder ::= Str(B) LBracket item_list RBracket { println!("folder ::= Str({}) {{ item_list }}",B); };
    object ::= class Name(B) { println!("object ::= class Name({})",B); };
    object ::= class Name params { println!("object ::= class Name params"); };
    object ::= class Asterisk params { println!("object ::= class * params"); };
    object ::= class params { println!("object ::= class params"); };
    object ::= class Name KwImport Str { println!("object ::= class Name KwImport Str"); };

    class ::= KwTex { println!("class ::= KwTex"); };
    class ::= KwFont { println!("class ::= KwFont"); };
    class ::= KwSprite { println!("class ::= KwSprite"); };
    class ::= KwIntMap { println!("class ::= KwIntMap"); };
    class ::= KwExtMap { println!("class ::= KwExtMap"); };

    params ::= LParen param_list RParen { println!("params ::= ( param_list )"); };
    param_list ::= param_list Comma param { println!("param_list ::= param_list , param"); };
    param_list ::= param { println!("param_list ::= param"); };

    param ::= Name(B) { println!("param ::= Name({})",B); };
    param ::= Name(B) value { println!("param ::= Name({}) value",B); };
    value ::= Str(B) { println!("value ::= Str({})",B); };
    value ::= Name(B) { println!("value ::= Name({})",B); };
    value ::= int_list { println!("value ::= int_list"); };
    value ::= valobj { println!("value ::= valobj"); };

    int_list ::= Int { println!("int_list ::= 1x Int"); };
    int_list ::= Int Int { println!("int_list ::= 2x Int"); };
    int_list ::= Int Int Int Int { println!("int_list ::= 4x Int"); };

    valobj ::= Name params { println!("valobj ::= Name params"); };

/*    package ::= Int;
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
    package ::= KwExtMap;*/

}

pub use parser::Parser;
pub use parser::Token;
