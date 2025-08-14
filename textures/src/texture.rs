use std::{fs, io::Write, path::Path};

use anyhow::Result;
use bincode::{Decode, Encode};

use crate::color::Color16;

#[derive(Decode, Encode)]
pub struct Texture {
    data: Vec<Color16>,
    pub width: u32,
    pub height: u32,
    pub transparent_color: Option<Color16>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Texture {
        Texture {
            width,
            height,
            transparent_color: None,
            data: vec![Color16(0); (width * height) as usize],
        }
    }

    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Texture> {
        let mut file = fs::File::open(filename)?;
        let config = bincode::config::standard().with_fixed_int_encoding();
        Ok(bincode::decode_from_std_read(&mut file, config)?)
    }

    pub fn set(&mut self, x: u32, y: u32, value: Color16) {
        self.data[(x + y * self.width) as usize] = value;
    }

    pub fn get(&self, x: u32, y: u32) -> Color16 {
        self.data[(x + y * self.width) as usize]
    }

    pub fn save<P: AsRef<Path>>(&self, filename: P) -> Result<()> {
        let mut file = fs::File::create(filename)?;
        let config = bincode::config::standard().with_fixed_int_encoding();
        bincode::encode_into_std_write(self, &mut file, config)?;
        Ok(())
    }

    pub fn save_bin<P: AsRef<Path>>(&self, filename: P) -> Result<()> {
        let mut file = fs::File::create(filename)?;
        file.write_all(&self.width.to_le_bytes())?;
        file.write_all(&self.height.to_le_bytes())?;
        if let Some(transp_color) = self.transparent_color {
            let yes = 1u8;
            file.write_all(&yes.to_le_bytes())?;
            file.write_all(&transp_color.0.to_le_bytes())?;
        } else {
            let no = 0u8;
            file.write_all(&no.to_le_bytes())?;
        }
        for pixel in &self.data {
            file.write_all(&pixel.0.to_le_bytes())?;
        }
        //file.write_all(&.is_some().to_le_bytes())?;
        Ok(())
    }
}
