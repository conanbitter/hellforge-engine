use image::Rgb;

use crate::color::Color16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBColor {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

const fn get_colors_rb() -> [i32; 32] {
    let mut result = [0; 32];
    let mut i = 0;
    while i < 32 {
        result[i] = ((i as f64) * 255.0 / 31.0) as i32;
        i += 1;
    }
    result
}

const fn get_colors_g() -> [i32; 64] {
    let mut result = [0; 64];
    let mut i = 0;
    while i < 64 {
        result[i] = ((i as f64) * 255.0 / 63.0) as i32;
        i += 1;
    }
    result
}

const COLORS_RB: [i32; 32] = get_colors_rb();
const COLORS_G: [i32; 64] = get_colors_g();

impl RGBColor {
    pub fn new(r: i32, g: i32, b: i32) -> RGBColor {
        RGBColor { r, g, b }
    }

    pub fn to16bit(&self) -> RGBColor {
        RGBColor {
            r: self.r.clamp(0, 255) / 8,
            g: self.g.clamp(0, 255) / 4,
            b: self.b.clamp(0, 255) / 8,
        }
    }

    pub fn to24bit(&self) -> RGBColor {
        RGBColor {
            r: COLORS_RB[self.r.clamp(0, 31) as usize],
            g: COLORS_G[self.g.clamp(0, 63) as usize],
            b: COLORS_RB[self.b.clamp(0, 31) as usize],
        }
    }
}

impl From<RGBColor> for Color16 {
    fn from(color: RGBColor) -> Self {
        let r = color.r.clamp(0, 31) as u16;
        let g = color.g.clamp(0, 63) as u16;
        let b = color.b.clamp(0, 31) as u16;
        let result = r << 11 | g << 5 | b;
        Color16(result)
    }
}

impl From<Color16> for RGBColor {
    fn from(color: Color16) -> Self {
        let r = (color.0 >> 11) as usize;
        let g = ((color.0 >> 5) & 0b111111) as usize;
        let b = (color.0 & 0b11111) as usize;
        RGBColor {
            r: COLORS_RB[r],
            g: COLORS_G[g],
            b: COLORS_RB[b],
        }
    }
}

impl From<&Rgb<u8>> for RGBColor {
    fn from(value: &Rgb<u8>) -> Self {
        RGBColor {
            r: value[0] as i32,
            g: value[1] as i32,
            b: value[2] as i32,
        }
    }
}
