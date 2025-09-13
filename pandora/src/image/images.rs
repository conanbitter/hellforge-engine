use crate::image::colors::Color16;

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
}
