use super::utils::*;
use super::{Material, ScatteredRay};
use crate::tracer::{Intersection, Ray, Vector3f};

// Lambertian (diffuse) Material.
// It can either scatter always and attenuate by its reflectance R, or it can scatter with no attenuation but absorb the fraction 1-R of the rays.
// Or it could be a mixture of those strategies.

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    albedo: Vector3f,
}

impl Lambertian {
    pub fn new(a: Vector3f) -> Lambertian {
        return Lambertian { albedo: a };
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &Intersection) -> Option<ScatteredRay> {
        let scatter = scatter_lambertian(self.albedo, hit);
        return Some(scatter);
    }
}
