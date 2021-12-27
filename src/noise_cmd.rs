use crate::tracer::material::noises::PerlinNoise;
use crate::tracer::Color;
use image::ImageBuffer;
use std::path::PathBuf;

pub fn render(
    output: PathBuf,
    scale: f64,
    frequency: f64,
    lacunarity: f64,
    persistency: f64,
    octaves: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let width: u32 = 400;
    let height: u32 = 300;

    let perlin = PerlinNoise::new(octaves, frequency, persistency, lacunarity);

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let u = x as f64 / width as f64;
        let v = y as f64 / height as f64;
        let n = perlin.noise2(u * scale, v * scale);

        let mut color = Color::new(n, n, n);
        color *= 255.99f64;

        image::Rgb([color.red as u8, color.green as u8, color.blue as u8])
    });

    match img.save(output.clone()) {
        Ok(_) => println!("Saved to file!"),
        Err(error) => println!("Oh noes: {}", error),
    }

    opener::open(output)?;

    Ok(())
}
