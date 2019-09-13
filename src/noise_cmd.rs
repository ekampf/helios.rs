use crate::tracer::Color;
use image::ImageBuffer;
use std::path::PathBuf;

pub fn render(output: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    opener::open(output);

    Ok(())
}
