use crate::color::Color16;

pub struct Texture {
    data: Vec<Color16>,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Texture {
        Texture {
            width,
            height,
            data: vec![Color16(0); (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, value: Color16) {
        self.data[(x + y * self.width) as usize] = value;
    }

    pub fn get(&self, x: u32, y: u32) -> Color16 {
        self.data[(x + y * self.width) as usize]
    }
}
