use std::fs;

use crate::project::tasks_from_string;

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

    let project_source = fs::read_to_string("test.pnd")?;

    //let project_source = String::from("123 453 \"Hel\\\\lo !\"  sd (tex) 23");

    let package = tasks_from_string(&project_source)?;

    println!("Package: {}", package.filename);
    println!("Tasks:");
    for task in package.tasks {
        println!("    {:?}", task);
    }

    Ok(())
}
