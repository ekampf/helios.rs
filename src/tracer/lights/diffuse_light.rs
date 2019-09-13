use crate::tracer::material::{Material, ScatteredRay, Texture};
use crate::tracer::{Intersection, Point3f, Ray, Vector3f};
use std::sync::Arc;

pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray_in: &Ray, hit: &Intersection) -> Option<ScatteredRay> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3f) -> Vector3f {
        self.texture.texture_value(u, v, p)
    }
}
