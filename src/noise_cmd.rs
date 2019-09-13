use crate::tracer::Color;
use image::ImageBuffer;
use std::path::PathBuf;

pub fn render(output: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    //    let perlin = OpenSimplex::new();
    //
    //    if let Some(fout) = output.to_str() {
    //        let mut b = PlaneMapBuilder::new(&perlin);
    //        return b.build().write_to_file(fout);
    //        println!("Oh yey");
    //    }

    //    let img = ImageBuffer::from_fn(400, 400, |x, y| {
    //        let x = x as f64;
    //        let y = y as f64;
    //
    //        let noise = perlin.get([x, y]);
    //
    //        let mut color = Color::new(0.5, 0.5, 0.5) * noise;
    //        color = color.sqrt();
    //        color *= 255.99f64;
    //
    //        image::Rgb([color.red as u8, color.green as u8, color.blue as u8])
    //    });

    //    match img.save(&output) {
    //        Ok(_) => println!("Saved to file!"),
    //        Err(error) => println!("Oh noes: {}", error),
    //    }

    opener::open(output);

    Ok(())
}
