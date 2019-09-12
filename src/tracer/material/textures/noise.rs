use crate::tracer::material::Texture;
use crate::tracer::{Point3f, Vector3f};
use cgmath::*;
use noise::{NoiseFn, Perlin};

pub struct NoiseTexture {
    scale: f64,
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            scale,
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn texture_value(&self, _u: f64, _v: f64, p: Point3f) -> Vector3f {
        let sp = p * self.scale;
        let v1 = vec3(1.0, 1.0, 1.0);
        v1 * self.noise.get([sp.x, sp.y, sp.z])
    }
}
