use std::{collections::HashMap, path::PathBuf};

use anyhow::anyhow;
use path_slash::PathBufExt;
use shared::DitheringMethod;

use crate::ast::{Node, PropConst, PropValue, Props};

#[derive(Debug)]
pub enum ResType {
    Texture,
    Font,
    Sprite,
    IntMap,
    ExtMap,
}

#[derive(Debug)]
enum TaskKind {
    TextureConvert(TextureParams),
    FontConvert(FontParams),
    SpriteConvert(SpriteParams),
    CopyFile(ResType),
}

#[derive(Debug)]
pub struct Task {
    name: Option<String>,
    src: String,
    dest: String,
    kind: TaskKind,
}

#[derive(Debug)]
struct TextureParams {
    transparent: bool,
    dithering: DitheringMethod,
}

#[derive(Debug)]
struct FontParams {
    transparent: bool,
    dithering: DitheringMethod,
    cols: u32,
    rows: u32,
    border_left: Option<u32>,
    border_right: Option<u32>,
    border_top: Option<u32>,
    border_bottom: Option<u32>,
    start_char: u32,
    end_char: u32,
    fallback_char: u32,
    letter_space: i32,
    line_height: i32,
}

#[derive(Debug)]
struct SpriteParams {
    transparent: bool,
    dithering: DitheringMethod,
    cols: u32,
    rows: u32,
    origin_x: i32,
    origin_y: i32,
    frame_time: f32,
}

impl Default for TextureParams {
    fn default() -> Self {
        Self {
            transparent: false,
            dithering: DitheringMethod::No,
        }
    }
}

impl TextureParams {
    pub fn apply(&mut self, params: &TaskParams) {
        if params.params.contains_key("transparent") {
            self.transparent = true;
        }

        if let Some(&PropValue::Const(kind)) = params.params.get("dither") {
            self.dithering = convert_dithering(kind);
        }
    }
}

impl Default for FontParams {
    fn default() -> Self {
        Self {
            transparent: true,
            dithering: DitheringMethod::No,
            cols: 16,
            rows: 16,
            border_left: None,
            border_right: None,
            border_top: None,
            border_bottom: None,
            start_char: 0,
            end_char: 255,
            fallback_char: 255,
            letter_space: 1,
            line_height: 0,
        }
    }
}

impl FontParams {
    pub fn apply(&mut self, params: &TaskParams) {
        if params.params.contains_key("transparent") {
            self.transparent = true;
        }

        if let Some(&PropValue::Const(kind)) = params.params.get("dither") {
            self.dithering = convert_dithering(kind);
        }

        if let Some(&PropValue::Int(val)) = params.params.get("cols") {
            self.cols = val as u32;
        }

        if let Some(&PropValue::Int(val)) = params.params.get("rows") {
            self.rows = val as u32;
        }

        if let Some(&PropValue::Int(val)) = params.params.get("start_char") {
            self.start_char = val as u32;
        }

        if let Some(&PropValue::Int(val)) = params.params.get("end_char") {
            self.end_char = val as u32;
        }

        if let Some(&PropValue::Int(val)) = params.params.get("fallback_char") {
            self.fallback_char = val as u32;
        }

        if let Some(&PropValue::Int(val)) = params.params.get("letter_space") {
            self.letter_space = val;
        }

        if let Some(&PropValue::Int(val)) = params.params.get("line_height") {
            self.line_height = val;
        }

        if let Some(borders) = params.params.get("borders") {
            match borders {
                PropValue::Const(PropConst::Auto) => {
                    self.border_left = None;
                    self.border_right = None;
                    self.border_top = None;
                    self.border_bottom = None;
                }
                &PropValue::Int(v) => {
                    self.border_left = Some(v as u32);
                    self.border_right = Some(v as u32);
                    self.border_top = Some(v as u32);
                    self.border_bottom = Some(v as u32);
                }
                &PropValue::Int2(v1, v2) => {
                    self.border_left = Some(v1 as u32);
                    self.border_right = Some(v2 as u32);
                    self.border_top = None;
                    self.border_bottom = None;
                }
                &PropValue::Int4(v1, v2, v3, v4) => {
                    self.border_left = Some(v1 as u32);
                    self.border_right = Some(v2 as u32);
                    self.border_top = Some(v3 as u32);
                    self.border_bottom = Some(v4 as u32);
                }
                _ => {}
            }
        }
    }
}

impl Default for SpriteParams {
    fn default() -> Self {
        Self {
            transparent: true,
            dithering: DitheringMethod::No,
            cols: 1,
            rows: 1,
            origin_x: 0,
            origin_y: 0,
            frame_time: 1.0,
        }
    }
}

impl SpriteParams {
    pub fn apply(&mut self, params: &TaskParams) {
        if params.params.contains_key("transparent") {
            self.transparent = true;
        }

        if let Some(&PropValue::Const(kind)) = params.params.get("dither") {
            self.dithering = convert_dithering(kind);
        }

        if let Some(&PropValue::Int(val)) = params.params.get("cols") {
            self.cols = val as u32;
        }

        if let Some(&PropValue::Int(val)) = params.params.get("rows") {
            self.rows = val as u32;
        }

        if let Some(&PropValue::Int2(vx, vy)) = params.params.get("origin") {
            self.origin_x = vx;
            self.origin_y = vy;
        }

        if let Some(&PropValue::Int(val)) = params.params.get("fps") {
            self.frame_time = 1.0 / (val as f32);
        }
    }
}

fn convert_dithering(input: PropConst) -> DitheringMethod {
    match input {
        PropConst::Ord4 => DitheringMethod::Ord4,
        PropConst::Ord8 => DitheringMethod::Ord8,
        PropConst::Fs => DitheringMethod::FS,
        _ => DitheringMethod::No,
    }
}

#[derive(Debug)]
pub struct PackageTask {
    pub filename: String,
    pub tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
struct TaskParams {
    src: PathBuf,
    dest: PathBuf,
    params: HashMap<String, PropValue>,
}

impl TaskParams {
    pub fn new() -> TaskParams {
        TaskParams {
            src: PathBuf::new(),
            dest: PathBuf::from("/"),
            params: HashMap::new(),
        }
    }

    pub fn append_props(&mut self, other: &Props, dest: Option<String>) {
        if let Some(dest_path) = dest {
            self.dest.push(dest_path)
        }
        for (key, value) in other {
            if key == "from" {
                if let PropValue::Str(path) = value {
                    self.src.push(path);
                }
            } else {
                self.params.insert(key.clone(), value.clone());
            }
        }
    }
}

fn process_node(node: &Node, package: &mut PackageTask, context: &TaskParams) {
    let mut own_context = context.clone();
    match node {
        Node::Folder(path, props, childs) => {
            if let Some(someprops) = props {
                own_context.append_props(someprops, Some(path.clone()));
            }
            for node in childs {
                process_node(node, package, &own_context);
            }
        }
        Node::Object(res_type, name, props) => {
            if let Some(someprops) = props {
                own_context.append_props(someprops, None);
            }
            let kind = match res_type {
                ResType::Texture => {
                    let mut tex_params = TextureParams::default();
                    tex_params.apply(&own_context);
                    TaskKind::TextureConvert(tex_params)
                }
                ResType::Font => {
                    let mut font_params = FontParams::default();
                    font_params.apply(&own_context);
                    TaskKind::FontConvert(font_params)
                }
                ResType::Sprite => {
                    let mut sprite_params = SpriteParams::default();
                    sprite_params.apply(&own_context);
                    TaskKind::SpriteConvert(sprite_params)
                }
                ResType::IntMap => TaskKind::CopyFile(ResType::IntMap),
                ResType::ExtMap => TaskKind::CopyFile(ResType::ExtMap),
            };
            package.tasks.push(Task {
                name: name.clone(),
                src: own_context.src.to_slash().unwrap().into_owned(),
                dest: own_context.dest.to_slash().unwrap().into_owned(),
                kind,
            });
        }
        _ => {}
    }
}

pub fn generate_package(root: &Node) -> anyhow::Result<PackageTask> {
    if let Node::Package(filename, props, childs) = root {
        let mut result = PackageTask {
            filename: filename.clone(),
            tasks: Vec::new(),
        };

        let mut params = TaskParams::new();
        if let Some(someprops) = props {
            params.append_props(someprops, None);
        }

        for node in childs {
            process_node(node, &mut result, &params);
        }

        Ok(result)
    } else {
        Err(anyhow!("Root in not a package"))
    }
}
