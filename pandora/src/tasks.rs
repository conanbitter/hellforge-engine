use std::path::PathBuf;

use shared::DitheringMethod;

#[derive(Debug)]
pub enum ResType {
    Texture,
    Font,
    Sprite,
    IntMap,
    ExtMap,
}

enum TaskKind {
    TextureConvert(TextureParams),
    FontConvert(FontParams),
    SpriteConvert(SpriteParams),
    CopyFile(ResType),
}

pub struct Task {
    name: String,
    src: PathBuf,
    dest: PathBuf,
    kind: TaskKind,
}

struct TextureParams {
    transparent: bool,
    dithering: DitheringMethod,
}

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
    line_space: i32,
}

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
            line_space: 0,
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
