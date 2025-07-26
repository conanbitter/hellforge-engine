use std::collections::HashSet;

use image::{RgbImage, RgbaImage};

use crate::{
    color::Color16,
    rgbcolor::{RGBColor, RGBPlane},
    texture::Texture,
};

struct BackgroundColor {
    used_colors: HashSet<RGBColor>,
}

impl BackgroundColor {
    fn new() -> BackgroundColor {
        BackgroundColor {
            used_colors: HashSet::new(),
        }
    }

    fn add(&mut self, color: RGBColor) {
        self.used_colors.insert(color);
    }

    const PRIMARY_COLORS: [RGBColor; 8] = [
        RGBColor::MAGENTA,
        RGBColor::CYAN,
        RGBColor::YELLOW,
        RGBColor::RED,
        RGBColor::BLUE,
        RGBColor::GREEN,
        RGBColor::WHITE,
        RGBColor::BLACK,
    ];

    fn find(&self) -> RGBColor {
        for prim_color in BackgroundColor::PRIMARY_COLORS {
            if !self.used_colors.contains(&prim_color) {
                return prim_color;
            }
        }

        for r in 0..31 {
            for g in 0..63 {
                for b in 0..31 {
                    let color = RGBColor::new(r, g, b);
                    if !self.used_colors.contains(&color) {
                        return color;
                    }
                }
            }
        }

        RGBColor::TRANSPARENT
    }
}

// region: posterize

pub fn convert_posterize(image: &RgbImage) -> Texture {
    let mut result = Texture::new(image.width(), image.height());

    for (x, y, color) in image.enumerate_pixels() {
        result.set(x, y, Color16::from(RGBColor::from(color).to16bit()));
    }

    result
}

pub fn convert_posterize_transparent(image: &RgbaImage) -> Texture {
    let mut result = Texture::new(image.width(), image.height());
    let mut bg_color_finder = BackgroundColor::new();

    for (_, _, color) in image.enumerate_pixels() {
        bg_color_finder.add(RGBColor::from(color).to16bit());
    }

    let bg_color = Color16::from(bg_color_finder.find());
    result.transparent_color = Some(bg_color);

    for (x, y, color) in image.enumerate_pixels() {
        if color[3] > 128 {
            result.set(x, y, Color16::from(RGBColor::from(color).to16bit()));
        } else {
            result.set(x, y, bg_color);
        }
    }

    result
}

// endregion

// region: floyd-steinberg

pub fn convert_fs(image: &RgbImage) -> Texture {
    let mut inner = RGBPlane::new(image.width(), image.height());
    let mut result = Texture::new(image.width(), image.height());
    for (x, y, color) in image.enumerate_pixels() {
        let original_color = RGBColor::from(color);
        let correction = inner.get(x, y);
        let old_color = RGBColor::new(
            ((original_color.r as f64 + correction.r as f64 / 16.0) as i32).clamp(0, 255),
            ((original_color.g as f64 + correction.g as f64 / 16.0) as i32).clamp(0, 255),
            ((original_color.b as f64 + correction.b as f64 / 16.0) as i32).clamp(0, 255),
        );
        let new_color = old_color.to16bit();
        result.set(x, y, Color16::from(new_color));
        let error = old_color - new_color.to24bit();
        if x < image.width() - 1 {
            inner.add(x + 1, y, error * 7);
        }
        if y < image.height() - 1 {
            if x > 0 {
                inner.add(x - 1, y + 1, error * 3);
            }
            inner.add(x, y + 1, error * 5);
            if x < image.width() - 1 {
                inner.add(x + 1, y + 1, error);
            }
        }
    }
    result
}

pub fn convert_fs_transparent(image: &RgbaImage) -> Texture {
    let mut inner = RGBPlane::new(image.width(), image.height());
    let mut result = Texture::new(image.width(), image.height());
    let mut bg_color_finder = BackgroundColor::new();

    for (x, y, color) in image.enumerate_pixels() {
        if color[3] < 128 {
            inner.set(x, y, RGBColor::TRANSPARENT);
            continue;
        }
        let original_color = RGBColor::from(color);
        let correction = inner.get(x, y);
        let old_color = RGBColor::new(
            ((original_color.r as f64 + correction.r as f64 / 16.0) as i32).clamp(0, 255),
            ((original_color.g as f64 + correction.g as f64 / 16.0) as i32).clamp(0, 255),
            ((original_color.b as f64 + correction.b as f64 / 16.0) as i32).clamp(0, 255),
        );
        let new_color = old_color.to16bit();
        bg_color_finder.add(new_color);
        result.set(x, y, Color16::from(new_color));
        let error = old_color - new_color.to24bit();
        if x < image.width() - 1 {
            inner.add(x + 1, y, error * 7);
        }
        if y < image.height() - 1 {
            if x > 0 {
                inner.add(x - 1, y + 1, error * 3);
            }
            inner.add(x, y + 1, error * 5);
            if x < image.width() - 1 {
                inner.add(x + 1, y + 1, error);
            }
        }
    }

    let bg_color = Color16::from(bg_color_finder.find());
    result.transparent_color = Some(bg_color);

    for y in 0..result.height {
        for x in 0..result.width {
            if inner.get(x, y) == RGBColor::TRANSPARENT {
                result.set(x, y, bg_color);
            }
        }
    }
    result
}

// endregion

// region: ordered

const BAYER_INT_4X4: [i32; 16] = [
    0, 8, 2, 10, //
    12, 4, 14, 6, //
    3, 11, 1, 9, //
    15, 7, 13, 5, //
];

const BAYER_INT_8X8: [i32; 64] = [
    0, 32, 8, 40, 2, 34, 10, 42, //
    48, 16, 56, 24, 50, 18, 58, 26, //
    12, 44, 4, 36, 14, 46, 6, 38, //
    60, 28, 52, 20, 62, 30, 54, 22, //
    3, 35, 11, 43, 1, 33, 9, 41, //
    51, 19, 59, 27, 49, 17, 57, 25, //
    15, 47, 7, 39, 13, 45, 5, 37, //
    63, 31, 55, 23, 61, 29, 53, 21, //
];

const fn for_bayer4() -> [f64; 64] {
    let mut result = [0.0; 64];

    let mut y = 0usize;
    while y < 8 {
        let mut x = 0usize;
        while x < 8 {
            let ix = x % 4;
            let iy = y % 4;
            let iind = ix + iy * 4;
            let ind = x + y * 8;
            result[ind] = (BAYER_INT_4X4[iind] as f64) / 16.0 - 0.5;
            x += 1
        }
        y += 1;
    }

    result
}

const fn for_bayer8() -> [f64; 64] {
    let mut result = [0.0; 64];

    let mut i = 0usize;
    while i < 64 {
        result[i] = (BAYER_INT_8X8[i] as f64) / 64.0 - 0.5;
        i += 1;
    }

    result
}

const BAYER_FLOAT_4X4: [f64; 64] = for_bayer4();

const BAYER_FLOAT_8X8: [f64; 64] = for_bayer8();

const RADIUS_RB: f64 = 255.0 / 31.0;
const RADIUS_G: f64 = 255.0 / 63.0;

fn get_wrapped(pattern: &[f64; 64], x: u32, y: u32) -> f64 {
    let x = (x % 8) as usize;
    let y = (y % 8) as usize;
    pattern[x + y * 8]
}

fn ordered_dithering(image: &RgbImage, pattern: &[f64; 64]) -> Texture {
    let mut result = Texture::new(image.width(), image.height());
    for (x, y, color) in image.enumerate_pixels() {
        let original_color = RGBColor::from(color);
        let correction = get_wrapped(pattern, x, y);
        let old_color = RGBColor::new(
            ((original_color.r as f64 + correction * RADIUS_RB) as i32).clamp(0, 255),
            ((original_color.g as f64 + correction * RADIUS_G) as i32).clamp(0, 255),
            ((original_color.b as f64 + correction * RADIUS_RB) as i32).clamp(0, 255),
        );
        let new_color = old_color.to16bit();
        result.set(x, y, Color16::from(new_color));
    }
    result
}

fn ordered_dithering_transparent(image: &RgbaImage, pattern: &[f64; 64]) -> Texture {
    let mut result = Texture::new(image.width(), image.height());
    let mut bg_color_finder = BackgroundColor::new();
    let mut mask = RGBPlane::new(image.width(), image.height());

    for (x, y, color) in image.enumerate_pixels() {
        if color[3] < 128 {
            mask.set(x, y, RGBColor::TRANSPARENT);
            continue;
        }

        let original_color = RGBColor::from(color);
        let correction = get_wrapped(pattern, x, y);
        let old_color = RGBColor::new(
            ((original_color.r as f64 + correction * RADIUS_RB) as i32).clamp(0, 255),
            ((original_color.g as f64 + correction * RADIUS_G) as i32).clamp(0, 255),
            ((original_color.b as f64 + correction * RADIUS_RB) as i32).clamp(0, 255),
        );
        let new_color = old_color.to16bit();
        bg_color_finder.add(new_color);
        result.set(x, y, Color16::from(new_color));
    }

    let bg_color = Color16::from(bg_color_finder.find());
    result.transparent_color = Some(bg_color);

    for y in 0..result.height {
        for x in 0..result.width {
            if mask.get(x, y) == RGBColor::TRANSPARENT {
                result.set(x, y, bg_color);
            }
        }
    }
    result
}

pub fn convert_ordered4(image: &RgbImage) -> Texture {
    ordered_dithering(image, &BAYER_FLOAT_4X4)
}

pub fn convert_ordered8(image: &RgbImage) -> Texture {
    ordered_dithering(image, &BAYER_FLOAT_8X8)
}

pub fn convert_ordered4_transparent(image: &RgbaImage) -> Texture {
    ordered_dithering_transparent(image, &BAYER_FLOAT_4X4)
}

pub fn convert_ordered8_transparent(image: &RgbaImage) -> Texture {
    ordered_dithering_transparent(image, &BAYER_FLOAT_8X8)
}
// endregion
