use image::RgbImage;

use crate::{color::Color16, rgbcolor::RGBColor, texture::Texture};

pub fn convert_posterize(image: &RgbImage) -> Texture {
    let mut result = Texture::new(image.width(), image.height());

    for (x, y, color) in image.enumerate_pixels() {
        result.set(x, y, Color16::from(RGBColor::from(color).to16bit()));
    }

    result
}
