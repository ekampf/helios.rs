use crate::tracer::{Intersection, Ray, Vector3f};

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
}
