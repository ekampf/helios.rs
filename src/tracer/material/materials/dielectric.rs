use super::utils::*;
use crate::tracer::material::{Material, ScatteredRay};
use crate::tracer::{Intersection, Ray};
use cgmath::*;
use rand::prelude::*;

// Lambertian (diffuse) Material.
// It can either scatter always and attenuate by its reflectance R, or it can scatter with no attenuation but absorb the fraction 1-R of the rays.
// Or it could be a mixture of those strategies.

#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
    reflective_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        return Dielectric {
            reflective_idx: ref_idx,
        };
    }

    /// Create a Dielectric material with reflective index of 1.3-1.7
    pub fn new_glass() -> Dielectric {
        let mut rng = rand::thread_rng();
        let reflective_idx: f64 = rng.gen_range(1.3, 1.7);
        return Dielectric::new(reflective_idx);
    }

    /// Create a Dielectric material with reflective index of 2.35-245
    #[allow(dead_code)]
    pub fn new_diamond() -> Dielectric {
        let mut rng = rand::thread_rng();
        let reflective_idx: f64 = rng.gen_range(2.35, 2.45);
        return Dielectric::new(reflective_idx);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &Intersection) -> Option<ScatteredRay> {
        let reflected = reflect(ray_in.direction, hit.normal);
        let attenuation = vec3(1.0, 1.0, 1.0);

        let outward_normal;
        let ni_over_nt;
        let mut cosine;

        let ray_dot_normal = ray_in.direction.dot(hit.normal);
        if ray_dot_normal > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.reflective_idx;

            cosine = ray_dot_normal / ray_in.direction.magnitude();
            cosine =
                (1.0 - self.reflective_idx * self.reflective_idx * (1.0 - cosine * cosine)).sqrt();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.reflective_idx;
            cosine = -ray_dot_normal / ray_in.direction.magnitude();
        }

        let scatter_ray_direction;
        if let Some(refracted) = refract(ray_in.direction, outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.reflective_idx);

            let mut rng = rand::thread_rng();
            let r: f64 = rng.gen();
            if r < reflect_prob {
                scatter_ray_direction = reflected;
            } else {
                scatter_ray_direction = refracted;
            }
        } else {
            scatter_ray_direction = reflected;
        }

        return Some(ScatteredRay {
            attenuation,
            ray: Ray::new(hit.point, scatter_ray_direction),
        });
    }
}
