use crate::tracer::material::{Material, ScatteredRay, SolidTexture, Texture};
use crate::tracer::{random_in_unit_sphere, Color, Intersection, Ray};
use std::sync::Arc;

// Lambertian (diffuse) Material.
// It can either scatter always and attenuate by its reflectance R, or it can scatter with no attenuation but absorb the fraction 1-R of the rays.
// Or it could be a mixture of those strategies.

#[derive(Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo }
    }
    pub fn from_constant(c: Color) -> Lambertian {
        let albedo = Arc::new(SolidTexture::new(c));
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &Intersection) -> Option<ScatteredRay> {
        let reflection = Ray::new(hit.point, hit.normal + random_in_unit_sphere());
        let (u, v) = hit.uv;
        let attenuation = self.albedo.texture_value(u, v, reflection.origin);

        let scatter = ScatteredRay {
            attenuation,
            ray: reflection,
        };

        Some(scatter)
    }
}
