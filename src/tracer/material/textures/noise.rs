use super::noises::PerlinNoise;
use crate::tracer::material::Texture;
use crate::tracer::{Color, Point3f, Vector3f};
use cgmath::*;

pub struct NoiseTexture {
    scale: f64,
    noise: PerlinNoise,
    color_start: Color,
    color_end: Color,
}

impl NoiseTexture {
    pub fn new(
        scale: f64,
        octaves: u32,
        freq: f64,
        pers: f64,
        lacu: f64,
        color_start: Color,
        color_end: Color,
    ) -> NoiseTexture {
        let perlin = PerlinNoise::new(octaves, freq, pers, lacu);

        NoiseTexture {
            scale,
            noise: perlin,
            color_start,
            color_end,
        }
    }

    #[allow(dead_code)]
    pub fn default() -> NoiseTexture {
        let perlin = PerlinNoise::default();

        NoiseTexture {
            scale: 1.0,
            noise: perlin,
            color_start: Color::black(),
            color_end: Color::white(),
        }
    }
}

impl Texture for NoiseTexture {
    fn texture_value(&self, _u: f64, _v: f64, p: Point3f) -> Vector3f {
        let sp = p * self.scale;
        let v1 = vec3(1.0, 1.0, 1.0);

        let noise_sine = (sp.z + 10.0 * self.noise.noise(p.to_vec())).sin();

        let n = v1 * 0.5 * (1.0 + noise_sine);
        let ni = vec3(1.0 - n.x, 1.0 - n.y, 1.0 - n.z);

        let cs = self.color_start;
        let ce = self.color_end;

        let r = cs.red * ni.x + ce.red * n.x;
        let g = cs.green * ni.y + ce.green * n.y;
        let b = cs.blue * ni.z + ce.blue * n.z;

        vec3(r, g, b)
    }
}
