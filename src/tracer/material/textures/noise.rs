use super::noises::PerlinNoise;
use crate::tracer::material::Texture;
use crate::tracer::{Point3f, Vector3f};

use cgmath::*;

pub struct NoiseTexture {
    scale: f64,
    noise: PerlinNoise,
}

impl NoiseTexture {
    pub fn new(scale: f64, octaves: u32, freq: f64, pers: f64, lacu: f64) -> NoiseTexture {
        let perlin = PerlinNoise::new(octaves, freq, pers, lacu);

        NoiseTexture {
            scale,
            noise: perlin,
        }
    }

    pub fn default() -> NoiseTexture {
        let perlin = PerlinNoise::default();

        NoiseTexture {
            scale: 1.0,
            noise: perlin,
        }
    }
}

impl Texture for NoiseTexture {
    fn texture_value(&self, _u: f64, _v: f64, p: Point3f) -> Vector3f {
        let sp = p * self.scale;
        let v1 = vec3(1.0, 1.0, 1.0);
        v1 * self.noise.noise(sp.to_vec())
    }
}
