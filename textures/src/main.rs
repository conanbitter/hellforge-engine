use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use image::{ImageReader, RgbImage};

use crate::{
    converters::{
        convert_fs, convert_fs_transparent, convert_ordered4, convert_ordered4_transparent, convert_ordered8,
        convert_ordered8_transparent, convert_posterize, convert_posterize_transparent,
    },
    rgbcolor::RGBColor,
    texture::Texture,
};

mod color;
mod converters;
mod rgbcolor;
mod texture;

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
enum DitheringMethod {
    No,
    FS,
    Ord4,
    Ord8,
}

#[derive(Parser, Debug)]
struct ArgMain {
    #[arg(required = true)]
    input: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(short, long)]
    transparent: bool,
    #[arg(short, long, default_value = "no")]
    dither: DitheringMethod,
}

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
    let args = ArgMain::parse();
    let outfile = if let Some(filename) = args.output {
        filename
    } else {
        let mut new_name = args.input.clone();
        new_name.set_extension("tex");
        new_name
    };
    print!("{:?} -> {:?}", args.input, outfile);
    if args.transparent || args.dither != DitheringMethod::No {
        print!(" ( ");
        if args.transparent {
            print!("Transparent");
            if args.dither != DitheringMethod::No {
                print!(", ");
            }
        }
        match args.dither {
            DitheringMethod::FS => print!("Floyd-Steinberg Dithering"),
            DitheringMethod::Ord4 => print!("Ordered 4x4 Dithering"),
            DitheringMethod::Ord8 => print!("Ordered 8x8 Dithering"),
            DitheringMethod::No => (),
        }
        print!(" ) ");
    }

    let tex = if args.transparent {
        let dithering_method = match args.dither {
            DitheringMethod::No => convert_posterize_transparent,
            DitheringMethod::FS => convert_fs_transparent,
            DitheringMethod::Ord4 => convert_ordered4_transparent,
            DitheringMethod::Ord8 => convert_ordered8_transparent,
        };
        let img = ImageReader::open(args.input)?.decode()?.to_rgba8();
        dithering_method(&img)?
    } else {
        let dithering_method = match args.dither {
            DitheringMethod::No => convert_posterize,
            DitheringMethod::FS => convert_fs,
            DitheringMethod::Ord4 => convert_ordered4,
            DitheringMethod::Ord8 => convert_ordered8,
        };
        let img = ImageReader::open(args.input)?.decode()?.to_rgb8();
        dithering_method(&img)
    };
    tex.save_bin(outfile)?;
    Ok(())
}
