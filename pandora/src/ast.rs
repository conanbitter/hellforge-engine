use crate::tasks::ResType;

#[derive(Debug, Clone, Copy)]
pub enum PropConst {
    Ord4,
    Ord8,
    Fs,
    None,
    Auto,
    Error,
}

#[derive(Debug, Clone)]
pub enum PropValue {
    Int(i32),
    Int2(i32, i32),
    Int4(i32, i32, i32, i32),
    Str(String),
    Const(PropConst),
    Empty,
}

pub type Props = Vec<(String, PropValue)>;

#[derive(Debug)]
pub enum Node {
    Package(String, Option<Props>, Vec<Node>),
    Folder(String, Option<Props>, Vec<Node>),
    Object(ResType, Option<String>, Option<Props>),
    ObjectImport(ResType, Option<String>, String),
}

pub fn const_from_string(name: String) -> PropConst {
    match name.to_lowercase().as_str() {
        "ord4" => PropConst::Ord4,
        "ord8" => PropConst::Ord8,
        "fs" => PropConst::Fs,
        "auto" => PropConst::Auto,
        "none" => PropConst::None,
        _ => PropConst::Error,
    }
}
