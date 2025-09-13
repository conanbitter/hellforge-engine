use image::{RgbImage, RgbaImage};

use crate::image::{
    colors::{Color16, ColorRGB, PlaneRGB},
    images::Image16,
};

pub fn convert_posterize(image: &RgbImage) -> Image16 {
    let mut result = Image16::new(image.width(), image.height());

    for (x, y, color) in image.enumerate_pixels() {
        result.set(x, y, Color16::from(ColorRGB::from(color)));
    }

    result
}

pub fn convert_fs(image: &RgbImage) -> Image16 {
    let mut inner = PlaneRGB::new(image.width(), image.height());
    let mut result = Image16::new(image.width(), image.height());
    for (x, y, color) in image.enumerate_pixels() {
        let original_color = ColorRGB::from(color);
        let correction = inner.get(x, y);
        let old_color = (original_color + correction).clamp();
        let new_color = Color16::from(old_color);
        result.set(x, y, new_color);
        let error = old_color - ColorRGB::from(new_color);
        if x < image.width() - 1 {
            inner.add(x + 1, y, error * (7.0 / 16.0));
        }
        if y < image.height() - 1 {
            if x > 0 {
                inner.add(x - 1, y + 1, error * (3.0 / 16.0));
            }
            inner.add(x, y + 1, error * (5.0 / 16.0));
            if x < image.width() - 1 {
                inner.add(x + 1, y + 1, error * (1.0 / 16.0));
            }
        }
    }
    result
}

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

const COLOR_RADIUS: ColorRGB = ColorRGB::new(255.0 / 31.0, 255.0 / 63.0, 255.0 / 31.0);

fn get_wrapped(pattern: &[f64; 64], x: u32, y: u32) -> f64 {
    let x = (x % 8) as usize;
    let y = (y % 8) as usize;
    pattern[x + y * 8]
}

fn ordered_dithering(image: &RgbImage, pattern: &[f64; 64]) -> Image16 {
    let mut result = Image16::new(image.width(), image.height());
    for (x, y, color) in image.enumerate_pixels() {
        let original_color = ColorRGB::from(color);
        let correction = get_wrapped(pattern, x, y);
        let old_color = (original_color + COLOR_RADIUS * correction).clamp();
        let new_color = Color16::from(old_color);
        result.set(x, y, new_color);
    }
    result
}
