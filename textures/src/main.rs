use anyhow::Result;
use image::{ImageReader, RgbImage};

use crate::{
    converters::{convert_fs, convert_fs_transparent, convert_posterize_transparent},
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
    /*let img = ImageReader::open("../assets/image1.png")?.decode()?.to_rgb8();
    let tex = convert_fs(&img);
    tex.save("../assets/image1.tex".to_string())?;

    let tex = Texture::from_file("../assets/image1.tex".to_string())?;
    save_texture(&tex, "../assets/image1_fromtex.png".to_string())?;*/

    let img = ImageReader::open("../assets/transp1.png")?.decode()?.to_rgba8();
    let tex = convert_fs_transparent(&img);
    save_texture(&tex, "../assets/transp1_res.png".to_string())?;

    /*let mut tex = Texture::new(2, 3);
    tex.set(0, 0, Color16(65001));
    tex.set(1, 0, Color16(65002));
    tex.set(0, 1, Color16(65003));
    tex.set(1, 1, Color16(65004));
    tex.set(0, 2, Color16(65005));
    tex.set(1, 2, Color16(65006));
    tex.save("../assets/test.tex".to_string())?;*/
    Ok(())
}
