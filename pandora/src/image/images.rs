use image::RgbImage;

use crate::image::colors::{Color16, ColorRGB};

pub struct Image16 {
    data: Vec<Color16>,
    pub width: u32,
    pub height: u32,
    pub transparent_color: Option<Color16>,
}

impl Image16 {
    pub fn new(width: u32, height: u32) -> Image16 {
        Image16 {
            width,
            height,
            transparent_color: None,
            data: vec![Color16(0); (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, value: Color16) {
        self.data[(x + y * self.width) as usize] = value;
    }

    pub fn get(&self, x: u32, y: u32) -> Color16 {
        self.data[(x + y * self.width) as usize]
    }

    pub fn debug_save(self, filename: String) -> anyhow::Result<()> {
        let mut img = RgbImage::new(self.width, self.height);
        for (x, y, color) in img.enumerate_pixels_mut() {
            let tex_color = ColorRGB::from(self.get(x, y));
            color[0] = f64::min(255.0, tex_color.r * 256.0) as u8;
            color[1] = f64::min(255.0, tex_color.g * 256.0) as u8;
            color[2] = f64::min(255.0, tex_color.b * 256.0) as u8;
        }
        img.save(filename)?;
        Ok(())
    }
}
