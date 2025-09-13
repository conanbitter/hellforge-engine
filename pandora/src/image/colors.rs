use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color16(pub u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorRGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl ColorRGB {
    pub fn new(r: f64, g: f64, b: f64) -> ColorRGB {
        ColorRGB { r, g, b }
    }
}

impl From<ColorRGB> for Color16 {
    fn from(color: ColorRGB) -> Self {
        let r = max(31, (color.r.clamp(0.0, 1.0) * 31.0) as u16);
        let g = max(63, (color.g.clamp(0.0, 1.0) * 63.0) as u16);
        let b = max(31, (color.b.clamp(0.0, 1.0) * 31.0) as u16);
        let result = r << 11 | g << 5 | b;
        Color16(result)
    }
}

impl Color16 {
    const fn new(r: u16, g: u16, b: u16) -> Color16 {
        Color16((r & 0b11111) << 11 | (g & 0b111111) << 5 | (b & 0b11111))
    }

    pub const BLACK: Color16 = Color16::new(0, 0, 0);
    pub const WHITE: Color16 = Color16::new(31, 63, 31);
    pub const RED: Color16 = Color16::new(31, 0, 0);
    pub const GREEN: Color16 = Color16::new(0, 63, 0);
    pub const BLUE: Color16 = Color16::new(0, 0, 31);
    pub const CYAN: Color16 = Color16::new(0, 63, 31);
    pub const MAGENTA: Color16 = Color16::new(31, 0, 31);
    pub const YELLOW: Color16 = Color16::new(31, 63, 0);
}
