use std::{collections::HashSet, i32, u64};

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

    fn find(&self) -> RGBColor {
        let mut result = RGBColor::new(0, 0, 0);
        let mut max_dist = 0;

        for r in 0..31 {
            for g in 0..63 {
                for b in 0..31 {
                    let color = RGBColor::new(r, g, b);
                    if !self.used_colors.contains(&color) {
                        let mut intrares = RGBColor::new(0, 0, 0);
                        let mut min_dist = i32::MAX;

                        for hs in self.used_colors.iter() {
                            let dist = RGBColor::distance_squared(color, *hs);
                            if dist < min_dist {
                                min_dist = dist;
                                intrares = *hs;
                            }
                        }

                        if min_dist > max_dist {
                            max_dist = min_dist;
                            result = intrares;
                        }
                    }
                }
            }
        }

        println!("Max distance: {}", max_dist);

        result
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

    fn find_fast(&self) -> RGBColor {
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

    let bg_color = Color16::from(bg_color_finder.find_fast());
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

fn mono(color: RGBColor) -> RGBColor {
    let avg = color.r + color.g + color.b;
    if avg > 128 * 3 {
        RGBColor::new(255, 255, 255)
    } else {
        RGBColor::new(0, 0, 0)
    }
}

pub fn convert_fs(image: &RgbImage) -> Texture {
    let mut inner = RGBPlane::new(image.width(), image.height());
    let mut result = Texture::new(image.width(), image.height());
    for (x, y, color) in image.enumerate_pixels() {
        let original_color = RGBColor::from(color);
        let correction = inner.get(x, y);
        let old_color = RGBColor::new(
            ((original_color.r as f32 + correction.r as f32 / 16.0) as i32).clamp(0, 255),
            ((original_color.g as f32 + correction.g as f32 / 16.0) as i32).clamp(0, 255),
            ((original_color.b as f32 + correction.b as f32 / 16.0) as i32).clamp(0, 255),
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
            ((original_color.r as f32 + correction.r as f32 / 16.0) as i32).clamp(0, 255),
            ((original_color.g as f32 + correction.g as f32 / 16.0) as i32).clamp(0, 255),
            ((original_color.b as f32 + correction.b as f32 / 16.0) as i32).clamp(0, 255),
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

    let bg_color = Color16::from(bg_color_finder.find_fast());
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
