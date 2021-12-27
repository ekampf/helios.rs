use crate::tracer::{Intersection, Point3f, Ray, Vector3f};
use cgmath::*;

/// The outgoing ray and attenuation (or weight) to assign the color of the traced ray.
/// - attenuation: The scaling of the reflection/refraction
/// - ray: The scattered ray
#[derive(Clone, Debug)]
pub struct ScatteredRay {
    pub attenuation: Vector3f,
    pub ray: Ray,
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray_in: &Ray, hit: &Intersection) -> Option<ScatteredRay>;

    fn emitted(&self, _ray_in: &Ray, _u: f64, _v: f64, _p: Point3f) -> Vector3f {
        vec3(0.0, 0.0, 0.0)
    }
}
