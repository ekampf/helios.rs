use super::ScatteredRay;
use crate::tracer::{random_point_on_unit_sphere, Intersection, Ray, Vector3f};
use cgmath::*;

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

pub fn reflect(v: Vector3f, normal: Vector3f) -> Vector3f {
    let d2 = v.dot(normal) * 2.0;
    return v - normal * d2;
}

pub fn refract(v: Vector3f, n: Vector3f, ni_over_nt: f64) -> Option<Vector3f> {
    let unit_v = v.normalize();
    let dt = unit_v.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted_ray = ni_over_nt * (unit_v - n * dt) - n * discriminant.sqrt();
        return Some(refracted_ray);
    }

    None
}

pub fn scatter_lambertian(albedo: Vector3f, hit: &Intersection) -> ScatteredRay {
    let reflection = Ray {
        origin: hit.point,
        direction: hit.normal + random_point_on_unit_sphere(),
    };

    ScatteredRay {
        attenuation: albedo,
        ray: reflection,
    }
}
