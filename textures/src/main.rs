use anyhow::Result;
use image::{ImageReader, RgbImage};

use crate::{
    converters::{convert_fs, convert_posterize},
    rgbcolor::RGBColor,
    texture::Texture,
};

mod color;
mod converters;
mod rgbcolor;
mod texture;

fn save_texture(texture: &Texture, filename: String) -> Result<()> {
    let mut img = RgbImage::new(texture.width, texture.height);
    for (x, y, color) in img.enumerate_pixels_mut() {
        let tex_color = RGBColor::from(texture.get(x, y));
        color[0] = tex_color.r as u8;
        color[1] = tex_color.g as u8;
        color[2] = tex_color.b as u8;
    }
    img.save(filename)?;
    Ok(())
}

fn main() -> Result<()> {
    let img = ImageReader::open("../assets/rainbow.png")?.decode()?.to_rgb8();
    let tex = convert_fs(&img);
    save_texture(&tex, "../assets/rainbow_tex.png".to_string())?;
    Ok(())
}
