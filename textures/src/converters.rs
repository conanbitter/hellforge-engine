use image::RgbImage;

use crate::{
    color::Color16,
    rgbcolor::{RGBColor, RGBPlane},
    texture::Texture,
};

pub fn convert_posterize(image: &RgbImage) -> Texture {
    let mut result = Texture::new(image.width(), image.height());

    for (x, y, color) in image.enumerate_pixels() {
        result.set(x, y, Color16::from(RGBColor::from(color).to16bit()));
    }

    result
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
