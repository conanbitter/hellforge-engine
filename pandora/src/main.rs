use std::fs;

use crate::project::project_from_file;

mod image;
mod project;

fn main() -> anyhow::Result<()> {
    /*match env::current_dir() {
        Ok(path) => println!("The current working directory is: {}", path.display()),
        Err(e) => eprintln!("Failed to get current working directory: {}", e),
    }

    let img = ImageReader::open("assets/transp1.png")?.decode()?.to_rgba8();
    let img16 = convert_ordered8_transparent(&img)?;
    img16.debug_save(String::from("assets/transp1_prc.png"))?;
    return Ok(());*/

    let package = project_from_file("pandora/test.pnd")?;

    println!("Package: {}", package.filename);
    println!("Tasks:");
    for task in package.tasks {
        println!("    {:?}", task);
    }

    Ok(())
}
