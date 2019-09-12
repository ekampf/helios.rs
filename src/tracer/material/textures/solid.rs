use crate::tracer::material::Texture;
use crate::tracer::{Color, Point3f, Vector3f};

pub struct SolidTexture {
    color: Color,
}

impl SolidTexture {
    pub fn new(color: Color) -> SolidTexture {
        SolidTexture { color }
    }
}

impl Texture for SolidTexture {
    fn texture_value(&self, _u: f64, _v: f64, _p: Point3f) -> Vector3f {
        self.color.to_vec3f()
    }
}
