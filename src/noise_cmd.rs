use crate::tracer::Color;
use image::ImageBuffer;
use simdnoise::*;
use std::path::PathBuf;

pub fn render(
    output: PathBuf,
    frequency: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let width: u32 = 400;
    let height: u32 = 300;

    let noise = NoiseBuilder::fbm_2d(width as usize, height as usize)
        .with_freq(frequency)
        .with_lacunarity(lacunarity)
        .with_gain(gain)
        .with_octaves(octaves)
        .generate_scaled(0.0, 1.0);

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let v = *noise.get((y * width + x) as usize).unwrap();

        let mut color = Color::new(v as f64, v as f64, v as f64);
        color *= 255.99f64;

        image::Rgb([color.red as u8, color.green as u8, color.blue as u8])
    });

    match img.save(output.clone()) {
        Ok(_) => println!("Saved to file!"),
        Err(error) => println!("Oh noes: {}", error),
    }

    opener::open(output);

    Ok(())
}
