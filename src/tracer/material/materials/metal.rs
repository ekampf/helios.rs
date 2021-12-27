use super::utils::reflect;
use crate::tracer::material::{Material, ScatteredRay};
use crate::tracer::math::random_in_unit_sphere;
use crate::tracer::{Intersection, Ray, Vector3f};
use cgmath::*;

// Lambertian (diffuse) Material.
// It can either scatter always and attenuate by its reflectance R, or it can scatter with no attenuation but absorb the fraction 1-R of the rays.
// Or it could be a mixture of those strategies.
#[derive(Copy, Clone, Debug)]
pub struct Metal {
    albedo: Vector3f,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3f, fuzz: f64) -> Metal {
        let fuzz = fuzz.min(1.0_f64);
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Intersection) -> Option<ScatteredRay> {
        let unit_in_direction = ray_in.direction.normalize();
        let reflected = reflect(unit_in_direction, hit.normal);

        let scattered = Ray::new(hit.point, reflected + random_in_unit_sphere() * self.fuzz);

        let is_scattered = scattered.direction.dot(hit.normal) > 0.0;
        if is_scattered {
            return Some(ScatteredRay {
                attenuation: self.albedo,
                ray: scattered,
            });
        }

        None
    }
}
