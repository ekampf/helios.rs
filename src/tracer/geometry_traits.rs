use crate::tracer::materials::Material;
use crate::tracer::{Intersection, Point3f, Ray};
use std::sync::Arc;

pub trait Intersectable: Sync {
    fn intersects(&self, ray: &Ray, dist_min: f64, dist_max: f64) -> Option<Intersection>;
}

// TODO: Boundable - returns BoundingBox

pub trait Geometry: Intersectable + Sync + Send {
    fn get_material(&self, point: Point3f) -> Box<Arc<dyn Material>>;

    fn primitives(&self) -> u64 {
        1
    }
}
