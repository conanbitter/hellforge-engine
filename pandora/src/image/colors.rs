use image::{Rgb, Rgba};
use std::{cmp::max, ops};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color16(pub u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorRGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl ColorRGB {
    pub const fn new(r: f64, g: f64, b: f64) -> ColorRGB {
        ColorRGB { r, g, b }
    }

    pub fn clamp(self) -> ColorRGB {
        ColorRGB {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }

    pub fn is_transparent(self) -> bool {
        self.r < 0.0
    }

    pub const TRANSPARENT: ColorRGB = ColorRGB::new(-1.0, 0.0, 0.0);
}

impl ops::AddAssign<ColorRGB> for ColorRGB {
    fn add_assign(&mut self, rhs: ColorRGB) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::Add<ColorRGB> for ColorRGB {
    type Output = ColorRGB;

    fn add(self, rhs: ColorRGB) -> Self::Output {
        ColorRGB {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Sub<ColorRGB> for ColorRGB {
    type Output = ColorRGB;

    fn sub(self, rhs: ColorRGB) -> Self::Output {
        ColorRGB {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::Mul<f64> for ColorRGB {
    type Output = ColorRGB;

    fn mul(self, rhs: f64) -> Self::Output {
        ColorRGB {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
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

impl From<Color16> for ColorRGB {
    fn from(value: Color16) -> Self {
        let r = value.0 >> 11;
        let g = (value.0 >> 5) & 0b111111;
        let b = value.0 & 0b11111;
        ColorRGB::new(r as f64 / 31.0, g as f64 / 63.0, b as f64 / 31.0).clamp()
    }
}

impl From<&Rgb<u8>> for ColorRGB {
    fn from(value: &Rgb<u8>) -> Self {
        ColorRGB {
            r: value[0] as f64 / 255.0,
            g: value[1] as f64 / 255.0,
            b: value[2] as f64 / 255.0,
        }
    }
}

impl From<&Rgba<u8>> for ColorRGB {
    fn from(value: &Rgba<u8>) -> Self {
        ColorRGB {
            r: value[0] as f64 / 255.0,
            g: value[1] as f64 / 255.0,
            b: value[2] as f64 / 255.0,
        }
    }
}

impl Color16 {
    pub const fn new(r: u16, g: u16, b: u16) -> Color16 {
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

pub struct PlaneRGB {
    data: Vec<ColorRGB>,
    pub width: u32,
    pub height: u32,
}

impl PlaneRGB {
    pub fn new(width: u32, height: u32) -> PlaneRGB {
        PlaneRGB {
            width,
            height,
            data: vec![ColorRGB::new(0.0, 0.0, 0.0); (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, value: ColorRGB) {
        self.data[(x + y * self.width) as usize] = value;
    }

    pub fn add(&mut self, x: u32, y: u32, value: ColorRGB) {
        self.data[(x + y * self.width) as usize] += value;
    }

    pub fn get(&self, x: u32, y: u32) -> ColorRGB {
        self.data[(x + y * self.width) as usize]
    }
}
